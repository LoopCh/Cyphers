use crate::block_encryptor::BlockEncryptor;
use crate::padding::Padding;
use crate::utils::{convert_array, xor};



pub trait EBC<const TEXT_BLOCK_SIZE: usize, const CIPHER_BLOCK_SIZE: usize>:
    BlockEncryptor<TEXT_BLOCK_SIZE, CIPHER_BLOCK_SIZE>
{

    fn EBC_encrypt(&self, data: &[u8], padding : &dyn Padding<TEXT_BLOCK_SIZE>) -> Vec<u8> {
        let data = padding.pud(data);
        let mut result = Vec::with_capacity(data.len());

        for chunk in data.chunks(TEXT_BLOCK_SIZE) {
            let mut temp_chunk = [0u8; TEXT_BLOCK_SIZE];
            temp_chunk.copy_from_slice(chunk);

            let encrypted_block = self.encrypt_block(temp_chunk);

            result.extend_from_slice(&encrypted_block);
        }

        result
    }

    fn EBC_decrypt(&self, data: &[u8], padding: &dyn Padding<CIPHER_BLOCK_SIZE>) -> Vec<u8> {
        let mut result = Vec::with_capacity(data.len());

        for chunk in data.chunks(CIPHER_BLOCK_SIZE) {
            let mut temp_chunk = [0u8; CIPHER_BLOCK_SIZE];
            temp_chunk.copy_from_slice(chunk);

            let encrypted_block = self.decrypt_block(temp_chunk);

            result.extend_from_slice(&encrypted_block);
        }

        padding.unpad(&result)

    }
}

pub trait CBC<const TEXT_BLOCK_SIZE: usize, const CIPHER_BLOCK_SIZE: usize>:
    BlockEncryptor<TEXT_BLOCK_SIZE, CIPHER_BLOCK_SIZE>
{

    /// CBC режим шифрования (сцепление шифровальных блоков).
    ///
    /// # Arguments
    ///
    /// * `data`: Массив байт для шифрования
    /// * `padding`: Тип паддинга
    /// * `initialize_vec`: Инициализирующий вектор
    ///
    /// returns: Vec<u8, Global>
    fn CBC_encrypt(&self, data: &[u8], padding: &dyn Padding<TEXT_BLOCK_SIZE>, mut initialize_vec: [u8; TEXT_BLOCK_SIZE]) -> Vec<u8> {
        let data = padding.pud(data);

        let mut result = Vec::with_capacity(data.len());

        for chunk in data.chunks(TEXT_BLOCK_SIZE) {
            let temp_chunk: [u8; TEXT_BLOCK_SIZE] = chunk.try_into().expect("Неправильный размер блока");
            let text_xor: [u8; TEXT_BLOCK_SIZE] = xor(&temp_chunk, &initialize_vec);

            initialize_vec = text_xor;

            result.extend_from_slice(&self.encrypt_block(text_xor));
        }

        result
    }
    fn CBC_decrypt(&self, data: &[u8], padding: &dyn Padding<CIPHER_BLOCK_SIZE>, mut initialize_vec: [u8; TEXT_BLOCK_SIZE]) -> Vec<u8> {

        let mut result = Vec::with_capacity(data.len());

        for chunk in data.chunks(CIPHER_BLOCK_SIZE) {
            let temp_chunk: [u8; CIPHER_BLOCK_SIZE] = chunk.try_into().expect("Неправильный размер блока");

            let mut decrypted_block:[u8; TEXT_BLOCK_SIZE] = self.decrypt_block(temp_chunk);
            let prev_decrypt_block = decrypted_block;

            decrypted_block = xor(&decrypted_block, &initialize_vec);

            initialize_vec = prev_decrypt_block;

            result.extend_from_slice(&decrypted_block);
        }

        padding.unpad(&result)
    }
}

/// Режим обратной связи по выходу
/// ```plaintext
/// один из вариантов использования симметричного блочного шифра.
/// Особенностью режима является то, что в качестве входных данных для алгоритма блочного шифрования не используется само сообщение.
/// Вместо этого блочный шифр используется для генерации псевдослучайного потока байтов, который с помощью операции XOR складывается с блоками открытого текста.
/// Подобная схема шифрования называется потоковым шифром
/// ```
pub trait OFB<const TEXT_BLOCK_SIZE: usize, const CIPHER_BLOCK_SIZE: usize>:
    BlockEncryptor<TEXT_BLOCK_SIZE, CIPHER_BLOCK_SIZE>
{
    fn OFB_encrypt(&self, data: &[u8], padding: &dyn Padding<TEXT_BLOCK_SIZE>, mut initialize_vec: [u8; TEXT_BLOCK_SIZE]) -> Vec<u8> {
        let data = padding.pud(&data);

        let mut result = Vec::with_capacity(data.len());

        for chunk in data.chunks(TEXT_BLOCK_SIZE) {
            let temp_chunk: [u8; TEXT_BLOCK_SIZE] = chunk.try_into().expect("Неверный размер блока");

            initialize_vec = convert_array(self.encrypt_block(initialize_vec));

            let encrypted_block = xor(&temp_chunk, &initialize_vec);
            result.extend_from_slice(&encrypted_block);
        }

        result
    }

    fn OFB_decrypt(&self, data: &[u8], padding: &dyn Padding<TEXT_BLOCK_SIZE>, mut initialize_vec: [u8; TEXT_BLOCK_SIZE]) -> Vec<u8> {

        let mut result = Vec::with_capacity(data.len());

        for chunk in data.chunks(CIPHER_BLOCK_SIZE) {
            let temp_chunk: [u8; CIPHER_BLOCK_SIZE] = chunk.try_into().expect("Неверный размер блока");

            initialize_vec = convert_array(self.encrypt_block(initialize_vec));

            let decrypted_block = xor(&temp_chunk, &initialize_vec);
            result.extend_from_slice(&decrypted_block);
        };

        padding.unpad(&result)
    }
}

/// Режим обратной связи по шифротексту
pub trait CFB<const TEXT_BLOCK_SIZE: usize, const CIPHER_BLOCK_SIZE: usize>:
    BlockEncryptor<TEXT_BLOCK_SIZE, CIPHER_BLOCK_SIZE>
{
    fn CFB_encrypt(&self, data: &[u8], padding: &dyn Padding<TEXT_BLOCK_SIZE>, mut initialize_vec: [u8; TEXT_BLOCK_SIZE]) -> Vec<u8> {
        let data = padding.pud(&data);

        let mut result = Vec::with_capacity(data.len());

        for chunk in data.chunks(TEXT_BLOCK_SIZE) {
            let temp_chunk: [u8; TEXT_BLOCK_SIZE] = chunk.try_into().expect("Неверный размер блока");

            let encrypted_block = xor(
                &convert_array(self.encrypt_block(initialize_vec)),
                &temp_chunk);

            initialize_vec = encrypted_block;
            result.extend_from_slice(&encrypted_block);
        }

        result

    }
    fn CFB_decrypt(&self, data: &[u8], padding: &dyn Padding<TEXT_BLOCK_SIZE>, mut initialize_vec: [u8; TEXT_BLOCK_SIZE]) -> Vec<u8> {
        let mut result = Vec::with_capacity(data.len());

        for chunk in data.chunks(CIPHER_BLOCK_SIZE) {
            let temp_chunk: [u8; CIPHER_BLOCK_SIZE] = chunk.try_into().expect("Неверный размер блока");

            initialize_vec = convert_array(self.encrypt_block(initialize_vec));

            let decrypted_block = xor(&temp_chunk, &initialize_vec);

            initialize_vec = convert_array(temp_chunk);

            result.extend_from_slice(&decrypted_block);
        };

        padding.unpad(&result)

    }

}

pub trait CTR<const TEXT_BLOCK_SIZE: usize, const CIPHER_BLOCK_SIZE: usize>:
    BlockEncryptor<TEXT_BLOCK_SIZE, CIPHER_BLOCK_SIZE> {

    fn CTR_encrypt(&self, data: &[u8], padding: &dyn Padding<TEXT_BLOCK_SIZE>, initialize_vec: [u8; TEXT_BLOCK_SIZE]) -> Vec<u8> {
        let data = padding.pud(data);
        let mut result = Vec::with_capacity(data.iter().len());

        let mut counter = [0u8; TEXT_BLOCK_SIZE];

        for chunk in data.chunks(TEXT_BLOCK_SIZE) {
            let counter_block = generate_counter_block(&initialize_vec, &counter); // счетчик
            let encrypted_counter = self.encrypt_block(counter_block); // шифруем счётчик
            let temp_chunk :[u8; TEXT_BLOCK_SIZE] = chunk.try_into().expect("Неверный размер блока"); // исходные данные
            let encrypted_block = xor(&encrypted_counter,&temp_chunk); // xor зашифрованного счётчика и текста

            result.extend_from_slice(&encrypted_block);
            increment_counter(&mut counter);
        };
        result
    }

    fn CTR_decrypt(&self, data: &[u8], padding: &dyn Padding<TEXT_BLOCK_SIZE>, initialize_vec: [u8; TEXT_BLOCK_SIZE]) -> Vec<u8> {
        let mut result = Vec::with_capacity(data.iter().len());

        let mut counter = [0u8; TEXT_BLOCK_SIZE];

        for chunk in data.chunks(TEXT_BLOCK_SIZE) {
            let counter_block = generate_counter_block(&initialize_vec, &counter);
            let encrypted_counter = self.encrypt_block(counter_block);
            let temp_chunk :[u8; TEXT_BLOCK_SIZE] = chunk.try_into().expect("Неверный размер блока");
            let encrypted_block = xor(&encrypted_counter,&temp_chunk);

            result.extend_from_slice(&encrypted_block);
            increment_counter(&mut counter);
        };
        padding.unpad(&result)
    }
}

fn generate_counter_block<const TEXT_BLOCK_SIZE: usize>(nonce: &[u8; TEXT_BLOCK_SIZE], counter: &[u8; TEXT_BLOCK_SIZE]) -> [u8; TEXT_BLOCK_SIZE] {
    let mut counter_block = [0u8; TEXT_BLOCK_SIZE];

    let part_size = TEXT_BLOCK_SIZE / 2;
    counter_block[..part_size].copy_from_slice(&nonce[..part_size]);

    counter_block[part_size..].copy_from_slice(&counter[part_size..]);

    counter_block // половина битов он инициализирующего вектора, другая половина от счётчика
}


/// Увеличивает значение счётчика
fn increment_counter<const TEXT_BLOCK_SIZE: usize>(counter: &mut [u8; TEXT_BLOCK_SIZE]) {
    for byte in counter.iter_mut().rev() {
        *byte = byte.wrapping_add(1);
        if * byte != 0 {
            break
        }
    }
}

