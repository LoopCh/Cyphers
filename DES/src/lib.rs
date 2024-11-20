pub mod utils;
mod tests;

use utils::permutations::*;
use utils::sboxes::*;
use utils::key_gen::*;
use utils::LEFT_SHIFTS;

use traits::block_encryptor::BlockEncryptor;
use traits::encryption_mode::{CBC, EBC};

pub struct DES {
    key: u64,
}


impl BlockEncryptor<8, 8> for DES {
    /// Основная функция шифрования блока размером 64 бита, на выходе 64 бита
    fn encrypt_block(&self, block: [u8; 8]) -> [u8; 8] {
        let block = u64::from_be_bytes(block);

        // Начальная перестановка
        let permuted_block = initial_permutation(block);

        let mut left = (permuted_block >> 32) as u32;
        let mut right = (permuted_block & 0xFFFFFFFF) as u32;

        // Генерация раундовых ключей
        let round_keys: [u64; 16] = key_generation(self.key);

        // 16 раундов
        for i in 0..16 {
            let prev_right = right;

            // Функция расширения с 32 бит до 48 бит
            let expanded_right = expand(right);

            // XOR c раундовым ключём
            let xor_result = expanded_right ^ round_keys[i];

            // Функия замены с жатием S
            let s_box_result = s_box(xor_result);

            // Функция перестановки P
            let permuted_s_box = permutation(s_box_result);

            right = left ^ permuted_s_box;
            left = prev_right;
        };

        // Отмена последнего обмена между left и right согласно структуре фейстиля
        let temp = left;
        left = right;
        right = temp;

        let final_result: u64 = ((left as u64) << 32) | right as u64;
        let final_permutation = final_permutation(final_result);
        final_permutation.to_be_bytes()
    }

    fn decrypt_block(&self, block: [u8; 8]) -> [u8; 8] {
        let block = u64::from_be_bytes(block);
        let permuted: u64 = initial_permutation(block);

        let mut left: u32 = (permuted >> 32) as u32;
        let mut right: u32 = (permuted & 0xFFFFFFFF) as u32;

        let round_keys: [u64; 16] = key_generation(self.key);

        for i in (0..16).rev() { // Раунды в обратном порядке
            let prev_right: u32 = right;

            let expanded_right: u64 = expand(right);

            let xor_result: u64 = expanded_right ^ round_keys[i];

            let s_box_result: u32 = s_box(xor_result);

            let permuted_s_box: u32 = permutation(s_box_result);

            right = left ^ permuted_s_box;
            left = prev_right;
        }

        let temp: u32 = left;
        left = right;
        right = temp;

        let final_result: u64 = ((left as u64) << 32) | right as u64;
        let final_permutation = final_permutation(final_result);
        final_permutation.to_be_bytes()
    }
}


impl DES {
    pub fn new(key: &[u8;8]) -> DES {
        let reduced_key = discard_every_eighth_bit(&key);

        DES {
            key: reduced_key
        }
    }
}

/// Отбрасывает последний бит каждого байта ключа
/// Например: byte = 11100101, result_byte = 11100100
fn discard_every_eighth_bit(input: &[u8; 8]) -> u64 {
    let mut result: u64 = 0;

    for &byte in input.iter() {
        result <<= 8;
        result |= (byte & 0xFE) as u64; // 0xFE = 11111110, отбрасывает последний бит
    }
    result
}

/// Функция перестановки IP к входным данным
fn initial_permutation(input_data: u64) -> u64 {
    let mut output: u64 = 0;
    for i in IP {
        output <<= 1;
        output |= (input_data >> (64 - i)) & 1;
    }
    output
}

/// Функция генерации раундовых ключей
fn key_generation(key: u64) -> [u64;16] {
    let mut round_keys: [u64; 16] = [0; 16];

    // Применяем перестановку PC1
    let mut permuted_choice_1: u64 = 0;
    for &i in PC1.iter() {
        permuted_choice_1 <<= 1;
        permuted_choice_1 |= (key >> (64 - i)) & 1;
    }

    let mut c: u32 = (permuted_choice_1 >> 28) as u32; // Левая половина ключа
    let mut d: u32 = (permuted_choice_1 & 0x0FFFFFFF) as u32; // Правая половина ключа

    for round in 0..16 {  // 16 раундов генерации ключа
        // Циклический сдвиг влево для раунда
        c = (c << LEFT_SHIFTS[round] | c >> (28 - LEFT_SHIFTS[round])) & 0x0FFFFFFF; // Оставляем только 28 бит
        d = (d << LEFT_SHIFTS[round] | d >> (28 - LEFT_SHIFTS[round])) & 0x0FFFFFFF;

        // Примененяем перестановку PC-2 и объеденяем с C и D
        let combined: u64 = ((c as u64) << 28) | d as u64; // Объеденяем C и D
        let mut round_key: u64 = 0;
        for &i in PC2.iter() {
            round_key <<= 1;
            round_key |= (combined >> (56 - i)) & 1; // Извлекаем и вставляем конкретный бит из 'combined' в 'round_key'.
        }

        round_keys[round] = round_key;
    }

    round_keys
}

// Функция расширения из 32 бит в 48 бит
fn expand(input: u32) -> u64 {
    let mut output: u64 = 0;
    for &i in E.iter() {
        output <<= 1;
        output |= ((input >> (32 - i)) & 1) as u64;
    }
    output
}

/// Функция S - замены с жатием.
/// Функция разбивает входное 48 битное число на 8 групп по 6 бит.
/// Первый и последний бит указывают номер строки.
/// 4 бита по центру номер столбца.
fn s_box(input: u64) -> u32 {
    let mut output: u32 = 0;

    for i in 0..8 {           // Итерируемся по 8 группам по 6 бит
        let chunk: u8 = ((input >> (42 - 6 * i)) & 0x3F) as u8; // Извлекаем 6 бит
        let row: usize = (((chunk & 0x20) >> 4) | (chunk & 0x01)) as usize; // Вычисляем индекс строки для поиска в S
        let col: usize = ((chunk & 0x1E) >> 1) as usize; // Вычисляем индекс столбца для поиска в S

        let s_value: u8 = S[i][row][col]; // Подставляем значение
        output = (output << 4) | s_value as u32; // Добавляем к output
    }

    output
}

/// Вспомогательное преобразование P
fn permutation(input: u32) -> u32 {
    let mut output: u32 = 0;
    for &i in P.iter() {
        output <<= 1;
        output |= (input >> (32 - i)) & 1;
    }
    output
}


/// Функция перестановки обратная к IP, для выходных данных
fn final_permutation(input: u64) -> u64 {
    let mut output: u64 = 0;
    for &i in FP.iter() {
        output <<= 1;
        output |= (input >> (64 - i)) & 1;
    }
    output
}



