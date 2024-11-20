use num_bigint::{BigInt, ToBigInt};
use num_traits::{ToPrimitive, Zero};
use super::{bits_to_byte};

pub struct GaluaField2 {
    irreducible_polynomial: u64,
}

impl GaluaField2 {
    pub fn new<const POLY_DEGREE: usize>(irreducible_poly: &[u8; POLY_DEGREE]) -> Self {
        Self {
            irreducible_polynomial: bits_to_byte::<POLY_DEGREE>(irreducible_poly),
        }
    }

    /// Умножает два полинома в двоичном представлении(Бинарная операция).
    pub fn multiply_polynomials_by_irreducible_poly<T: ToBigInt>(&self, a: &T, b: &T) -> u8 {
        let a = a.to_bigint().unwrap();
        let b = b.to_bigint().unwrap();

        let mut result = BigInt::zero();
        let mut a_shifted = a;

        // Используем битовые операции для умножения
        for i in 0..b.bits() {
            if b.bit(i) {
                result ^= &a_shifted; // Сложение в GF(2) - это XOR
            }
            a_shifted <<= 1; // Сдвигаем a влево (умножаем на x)
        }

        // Получаем двоичный остаток после деления на неприводимый полином
        Self::get_binary_remainder(result, self.irreducible_polynomial.into()).to_u8().unwrap()
    }

    /// Вычисляет двоичный остаток от операции деления.
    fn get_binary_remainder(mut remainder: BigInt, divisor: BigInt) -> u64 {
        let divisor_bit_length = divisor.bits();

        while remainder.bits() >= divisor_bit_length {
            // Выравниваем делитель с текущим остатком
            let shift = remainder.bits() - divisor_bit_length;
            let shifted_divisor = &divisor << shift;
            remainder ^= &shifted_divisor; // Вычитание в GF(2) также является XOR
        }
        remainder.to_u64().unwrap()
    }
    //00001001 x^8 + x^4 + x^3 + x + 1

    // println!("{:04b}", 0b10000000 ^ (0b10011 << 3));
    // println!("{:04b}", 0b110000 ^ (0b10011 << 1));  Пример функции module redaction, которая берёт остаток от деления двочиного числа на двоичное число
    // println!("{:04b}", 0b10110 ^ 0b10011);
}
