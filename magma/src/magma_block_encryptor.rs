use traits::block_encryptor::{BlockEncryptor};
use crate::utils::{mod32add, PI};

const BLOCK_SIZE: usize = 8;
const SHIFT_AMOUNT: usize = 4;
const ROTATE_LEFT: usize = 11;
const ROTATE_RIGHT: usize = 21;
pub struct MagmaBlockEncryptor {
    key: [u8;32],
}

impl BlockEncryptor<8, 8> for MagmaBlockEncryptor {
    fn encrypt_block(&self, block: [u8; 8]) -> [u8; 8] {
        self.process_block(block, true)
    }

    fn decrypt_block(&self, block: [u8; 8]) -> [u8; 8] {
        self.process_block(block, false)
    }
}


impl MagmaBlockEncryptor {

    fn process_block(&self, block: [u8; 8], is_encrypt: bool) -> [u8; 8] {
        // Разделение на левую и правую часть
        let block = u64::from_be_bytes(block);
        let mut left_part = (block >> 32) as u32;
        let mut right_part = (block & 0xFFFFFFFF) as u32;

        // Получение раундовых ключей
        let round_keys = self.set_keys();
        let rounds = 31;

        // Раунды
        for i in 0..rounds {
            let temp = right_part;
            let key_index = if is_encrypt { i } else { rounds - i };
            right_part = left_part ^ Self::f(round_keys[key_index], right_part);
            left_part = temp;
        }

        // Последний раунд
        let final_key_index = if is_encrypt { rounds } else { 0 };
        left_part ^= Self::f(round_keys[final_key_index], right_part);

        let mut result = [0u8; 8];
        result[0..4].copy_from_slice(&left_part.to_be_bytes());
        result[4..8].copy_from_slice(&right_part.to_be_bytes());

        // println!("result");
        // for element in result {
        //     print!("{:02x}", element);
        // }

        result
    }

    pub fn set_keys(&self) -> [u32;32] {

        let round_keys = self.key
            .chunks(4)
            .map(|chunk|{
            u32::from_be_bytes(chunk.try_into().expect("Invalid chunk size"))

        }).collect::<Vec<u32>>();
        let mut results_key = [0u32;32];

        for i in 0..24 {
            results_key[i] = round_keys[i % 8];
        };
        for i in (0..=7).rev() {
            results_key[32 - i - 1] = round_keys[i];
        };
        results_key
    }

    pub fn new(key: &[u8;32]) -> MagmaBlockEncryptor {
        Self {
            key: *key,
        }
    }

    // Функция преобразования f
    pub fn f(round_key: u32, right_part: u32) -> u32  {

            // Сложение по модулю 2^32
            let result_addition = mod32add(right_part, round_key);
            // Получение 8 блоков по 4 бита
            let micro_blocks = Self::extract_4_bit_block(result_addition);
            // Преобразование подстановки
            let pi_integrated_blocks = Self::integrate_with_pi(micro_blocks);
            // Получение числа обратно из битов
            let combined = Self::combine_blocks(pi_integrated_blocks);
            // Циклический сдвиг влево на 11
            Self::rotate_combined(combined, ROTATE_LEFT, ROTATE_RIGHT)
    }

    fn extract_4_bit_block(value: u32) -> [u32; 8] {
        let mut blocks = [0; 8];
        for i in 0..8 {
            blocks[BLOCK_SIZE - 1 - i] = (value >> (i * SHIFT_AMOUNT)) & 0xf;
        }
        blocks
    }

    fn integrate_with_pi(micro_blocks: [u32; 8]) -> [u8; 8] {
        let mut integrated_blocks = [0; 8];
        for i in 0..8 {
            integrated_blocks[i] = PI[i][micro_blocks[i] as usize];
        }
        integrated_blocks
    }

    fn combine_blocks(pi_integrated_blocks: [u8; 8]) -> u32 {
        let mut combined = 0;
        for block in pi_integrated_blocks {
            combined = (combined << 4) | (block as u32);
        }
        combined
    }

    fn rotate_combined(value: u32, left: usize, right: usize) -> u32 {
        (value << left) | (value >> right)
    }

}

