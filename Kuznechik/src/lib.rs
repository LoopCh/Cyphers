pub mod utils;

use utils::gfield::GaluaField2;
use traits::block_encryptor::BlockEncryptor;
use traits::encryption_mode::{CBC, CFB, CTR, EBC, OFB};
use crate::utils::consts::{L_VEC, PI, PI_REV};

pub struct Kuznechik {
    key: [u8; 32]
}

impl EBC<16, 16> for Kuznechik{}
impl CBC<16, 16> for Kuznechik{}
impl OFB<16, 16> for Kuznechik{}
impl CFB<16, 16> for Kuznechik{}
impl CTR<16, 16> for Kuznechik{}

impl BlockEncryptor<16, 16> for Kuznechik {
    fn encrypt_block(&self, block: [u8; 16]) -> [u8; 16] {
        let round_keys = self.generate_round_keys();

        let mut state = block;

        for i in 0..9 {
            transform_x(&mut state, &round_keys[i]);
            transform_s(&mut state);
            transform_l(&mut state);
        }

        transform_x(&mut state, &round_keys[9]);

        state
    }
    fn decrypt_block(&self, block: [u8; 16]) -> [u8; 16] {
        let round_keys = self.generate_round_keys();

        let mut state = block;

        for i in (1..=9).rev() {
            transform_x(&mut state, &round_keys[i]);
            rev_transform_l(&mut state);
            rev_transform_s(&mut state);
        };
        transform_x(&mut state,&round_keys[0]);

        state
    }
}

impl Kuznechik {
    pub fn new(key: [u8;32]) -> Self {
        Self {
            key
        }
    }

    fn generate_round_keys(&self) -> [[u8;16]; 10] {
        // получаем итерационные константы
        let iter_consts = get_array_with_c();

        let mut round_keys = [[0u8;16];10];

        let mut left_part = [0u8;16];
        left_part.copy_from_slice(&self.key[..16]);

        let mut right_part = [0u8;16];
        right_part.copy_from_slice(&self.key[16..]);

        round_keys[0] = left_part;
        round_keys[1] = right_part;

        for i in 0..4 { // 4 раунда, т.к. генерируется по 2 ключа сразу (т.е. 2 * 4 = 8) а два предыдущих ключа взяли из мастер ключа
            // 8 раундов сети фейстиля
            for j in 0..8 {
                let parts = feistel_network(&left_part, &right_part, &iter_consts[j + 8 * i]);
                left_part = parts.0;
                right_part = parts.1;
            };
            round_keys[2 * i + 2] = left_part;
            round_keys[2 * i + 3] = right_part
        };
        round_keys
    }
}

fn feistel_network(left_part: &[u8;16], right_part: &[u8;16], iter_const: &[u8;16]) -> ([u8;16], [u8;16]) {
    let mut state = left_part.clone();
    // xor ключа и итерационной константы
    transform_x(&mut state, iter_const);
    transform_s(&mut state);
    transform_l(&mut state);

    transform_x(&mut state, right_part);

    (state.clone(), left_part.clone())
}

/// XOR блока с ключем, в методичке это X преобразование
fn transform_x(state: &mut [u8;16], key: &[u8;16]) {
    for (s, k) in state.iter_mut().zip(key) {
        *s ^= k;
    }
}

/// S преобразование - используется таблица подстановки PI(π)
fn transform_s(state: &mut [u8;16]) {
    for byte in state.iter_mut() {
        *byte = PI[*byte as usize];
    }
}

/// Обратное S преобразование
fn rev_transform_s(state: &mut [u8;16]) {
    for byte in state.iter_mut() {
        *byte = PI_REV[*byte as usize]
    }
}

/// L преобразование
fn transform_l(state: &mut [u8;16]) {
    for _ in 0..16 {
        transform_r(state);
    }
}

/// Обратное L преобразование
fn rev_transform_l(state: &mut [u8;16]) {
    for _ in 0..16 {
        rev_transform_r(state)
    }
}

/// R преобразование
fn transform_r(state: &mut [u8;16]) {
    let new_high_byte = transform_linear(state);
    state.rotate_right(1);
    state[0] = new_high_byte;
}

/// Обратное R преобразование
fn rev_transform_r(state: &mut [u8;16]) {
    state.rotate_left(1);
    state[15] = transform_linear(state);
}

/// Функция вычисления нового байта
fn transform_linear(state: &[u8;16]) -> u8 {
    let _gfield = GaluaField2::new(&[1,1,1,0,0,0,0,1,1]);

    let mut result: u8 = 0;

    for (byte, l) in state.iter().rev().zip(L_VEC) {
        result ^= gf_multiply(*byte, l)
    };

    result
}

fn gf_multiply(mut a: u8, mut b: u8) -> u8 {

    let mut result: u8 = 0;
    let mut high_bit: u8;

    for _ in 0..8 {
        if b & 0b00000001 == 0b00000001 {

            result ^= a;

        }
        high_bit = a & 0b10000000;

        a <<= 1;
        if high_bit == 0b10000000 {

            a ^= 0b11000011;

        }
        b >>= 1;
    }

    result
}

/// Функция генерации констант Ci
fn get_array_with_c() -> [[u8;16]; 32] {
    let mut iter_c = [[0;16];32];

    for i in 0..32 {
        let mut iter_num = [0u8;16];
        iter_num[15] = (i + 1) as u8;
        transform_l(&mut iter_num);
        iter_c[i] = iter_num;
    };
    iter_c
}
