pub mod utils;
use traits::block_encryptor::BlockEncryptor;
use crate::utils::consts::{INVSBOX, RCON, SBOX};
use crate::utils::gfield::GaluaField2;

pub struct AES {
    key: [u8; 16],
}

impl BlockEncryptor<16,16> for AES {
    fn encrypt_block(&self, block: [u8; 16]) -> [u8; 16] {
        let mut state = block;

        let round_keys = self.generate_round_keys();
        add_round_key(&mut state, &round_keys[0]);
        println!("{:x?}", &round_keys[0]);
        for round in round_keys.iter().take(10).skip(1) {
            println!("{:X?}", &round);
            sub_bytes(&mut state);
            shift_rows(&mut state);
            mix_cols(&mut state);
            add_round_key(&mut state, round);
        }

        sub_bytes(&mut state);
        shift_rows(&mut state);
        println!("{:x?}", &round_keys[10]);
        add_round_key(&mut state, &round_keys[10]);

        state
    }
    fn decrypt_block(&self, block: [u8; 16]) -> [u8; 16] {
        let mut state = block;
        let round_keys = self.generate_round_keys();
        add_round_key(&mut state, &round_keys[10]);
        for round in round_keys.iter().take(10).skip(1).rev() {
            inv_shift_rows(&mut state);
            inv_sub_bytes(&mut state);
            add_round_key(&mut state, round);
            inv_mix_cols(&mut state);
        }
        inv_shift_rows(&mut state);
        inv_sub_bytes(&mut state);
        add_round_key(&mut state, &round_keys[0]);
        state
    }
}

impl AES {
    pub fn new(key: [u8; 16]) -> AES {

        AES {
            key: key,
        }
    }

    /// Функция генерации раундовых ключей для алгоритма AES
    fn generate_round_keys(&self) -> [[u8;16]; 11] {
        let mut round_keys = [[0u8; 16]; 11];
        // Первый раундовый ключ копируется из исходного ключа
        round_keys[0].copy_from_slice(&self.key);
        let mut temp = [0u8; 4];

        // Генерация остальных 10 ключей
        for i in 1..11 {
            // Извлекаем последние 4 байта предыдущего раундового ключа
            temp.copy_from_slice(&round_keys[i-1][12..16]);

            // Сдвигаем байты влево на 1 позициую
            temp.rotate_left(1);

            // Каждый байт преобразуется с использованием SBOX

            for byte in &mut temp {
                *byte = SBOX[*byte as usize];
            }

            // Первый байт XOR'ится с соответствующим значением из RCON
            temp[0] ^= RCON[i - 1];

            //println!("g: {:}", u32::from_be_bytes(temp));

            // Генерация раундового ключа путем XOR с предыдущим ключём
            for (j, t) in temp.iter().enumerate() {
                round_keys[i][j] = round_keys[i - 1][j] ^ t;
                round_keys[i][j + 4] = round_keys[i - 1][j + 4] ^ round_keys[i][j];
                round_keys[i][j + 8] = round_keys[i - 1][j + 8] ^ round_keys[i][j + 4];
                round_keys[i][j + 12] = round_keys[i - 1][j + 12] ^ round_keys[i][j + 8];
            }
        }

        round_keys
    }
}

/// XOR исходного блока с раундовым ключем
fn add_round_key(block: &mut [u8;16], key: &[u8;16]) {
    for (s, k) in block.iter_mut().zip(key.iter()) {
        *s ^= k;
    }
}

/// Подстановка из SBOX
fn sub_bytes(state: &mut [u8; 16]) {
    for byte in state.iter_mut() {
        *byte = SBOX[*byte as usize];
    }
}

/// Обртаное преобразовние
fn inv_sub_bytes(state: &mut [u8; 16]) {
    for byte in state.iter_mut() {
        *byte = INVSBOX[*byte as usize]
    }
}

/// Сдвиг строк
/// ```plaintext
/// [ 0,  1,  2,  3 ]  ->  [ 0,  1,  2,  3 ] - Первая строка не сдвигается
/// [ 4,  5,  6,  7 ]  ->  [ 5,  6,  7,  4 ] - Вторая строка на 1 позиции влево
/// [ 8,  9, 10, 11 ]  ->  [10, 11,  8,  9 ] - Третья строка на 2 позиции влево
/// [12, 13, 14, 15 ]  ->  [15, 12, 13, 14 ] - Четвертая строка на 3 позиции влево
/// ```
fn shift_rows(state: &mut [u8; 16]) {
    let temp = *state;
    state[0] = temp[0];
    state[1] = temp[5];
    state[2] = temp[10];
    state[3] = temp[15];
    state[4] = temp[4];
    state[5] = temp[9];
    state[6] = temp[14];
    state[7] = temp[3];
    state[8] = temp[8];
    state[9] = temp[13];
    state[10] = temp[2];
    state[11] = temp[7];
    state[12] = temp[12];
    state[13] = temp[1];
    state[14] = temp[6];
    state[15] = temp[11];
}

/// Обратный сдвиг строк
fn inv_shift_rows(state: &mut [u8; 16]) {
    let temp = *state;
    state[0] = temp[0];
    state[4] = temp[4];
    state[8] = temp[8];
    state[12] = temp[12];
    state[1] = temp[13];
    state[2] = temp[10];
    state[3] = temp[7];
    state[5] = temp[1];
    state[6] = temp[14];
    state[7] = temp[11];
    state[9] = temp[5];
    state[10] = temp[2];
    state[11] = temp[15];
    state[13] = temp[9];
    state[14] = temp[6];
    state[15] = temp[3];
}

/// Функция берёт каждый столбец состояния и применяет к нему линейное преобразование, которое включает умножение
/// байтов в поле (GF(2^8)).
fn mix_cols(state: &mut [u8; 16]) {
    let gfield = GaluaField2::new(&[1, 0, 0, 0, 1, 1, 0, 1, 1]); // x^8 + x^4 + x^3 + x + 1

    let temp = *state;
    for col in 0..4 {
        let offset = col * 4;
        let s = [
            temp[offset],
            temp[offset + 1],
            temp[offset + 2],
            temp[offset + 3],
        ];

        state[offset] =
            gfield.multiply_polynomials_by_irreducible_poly(&s[0], &0x02)
                ^ gfield.multiply_polynomials_by_irreducible_poly(&s[1], &0x03)
                ^ s[2]
                ^ s[3];
        state[offset + 1] =
                s[0]
                ^ gfield.multiply_polynomials_by_irreducible_poly(&s[1], &0x02)
                ^ gfield.multiply_polynomials_by_irreducible_poly(&s[2], &0x03)
                ^ s[3];
        state[offset + 2] =
                s[0]
                ^ s[1]
                ^ gfield.multiply_polynomials_by_irreducible_poly(&s[2], &0x02)
                ^ gfield.multiply_polynomials_by_irreducible_poly(&s[3], &0x03);
        state[offset + 3] =
                gfield.multiply_polynomials_by_irreducible_poly(&s[0], &0x03)
                ^ s[1]
                ^ s[2]
                ^ gfield.multiply_polynomials_by_irreducible_poly(&s[3], &0x02);
    }
}

fn inv_mix_cols(state: &mut [u8; 16]) {
    let gfield = GaluaField2::new(&[1, 0, 0, 0, 1, 1, 0, 1, 1]);

    let temp = *state;
    for col in 0..4 {
        let offset = col * 4;
        let s = [
            temp[offset],
            temp[offset + 1],
            temp[offset + 2],
            temp[offset + 3],
        ];

        state[offset] =
            gfield.multiply_polynomials_by_irreducible_poly(&s[0], &0x0e)
                ^ gfield.multiply_polynomials_by_irreducible_poly(&s[1], &0x0b)
                ^ gfield.multiply_polynomials_by_irreducible_poly(&s[2], &0x0d)
                ^ gfield.multiply_polynomials_by_irreducible_poly(&s[3], &0x09);
        state[offset + 1] =
            gfield.multiply_polynomials_by_irreducible_poly(&s[0], &0x09)
                ^ gfield.multiply_polynomials_by_irreducible_poly(&s[1], &0x0e)
                ^ gfield.multiply_polynomials_by_irreducible_poly(&s[2], &0x0b)
                ^ gfield.multiply_polynomials_by_irreducible_poly(&s[3], &0x0d);
        state[offset + 2] =
            gfield.multiply_polynomials_by_irreducible_poly(&s[0], &0x0d)
                ^ gfield.multiply_polynomials_by_irreducible_poly(&s[1], &0x09)
                ^ gfield.multiply_polynomials_by_irreducible_poly(&s[2], &0x0e)
                ^ gfield.multiply_polynomials_by_irreducible_poly(&s[3], &0x0b);
        state[offset + 3] =
            gfield.multiply_polynomials_by_irreducible_poly(&s[0], &0x0b)
                ^ gfield.multiply_polynomials_by_irreducible_poly(&s[1], &0x0d)
                ^ gfield.multiply_polynomials_by_irreducible_poly(&s[2], &0x09)
                ^ gfield.multiply_polynomials_by_irreducible_poly(&s[3], &0x0e);
    }
}





