use num_bigint::{BigInt, ToBigInt};
use num_traits::{ToPrimitive, Zero, One};
use super::{bits_to_byte, leading_zeros};

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
        let mut a_shifted = a.clone();
        let mut b_temp = b.clone();

        while b_temp > BigInt::zero() {
            // Если текущий бит b равен 1, добавляем a_shifted к результату
            if &b_temp & BigInt::one() == BigInt::one() {
                result ^= &a_shifted; // Сложение в GF(2) - это XOR
            }

            // Сдвигаем a влево (умножаем на x)
            a_shifted <<= 1;

            // Сдвигаем b вправо
            b_temp >>= 1;
        }

        // Получаем двоичный остаток после деления на неприводимый полином
        Self::get_binary_remainder(result, self.irreducible_polynomial.into()).to_u8().unwrap()
    }

    /// Вычисляет двоичный остаток от операции деления.
    fn get_binary_remainder(mut remainder: BigInt, mut divisor: BigInt) -> u64 {
        while &divisor != &BigInt::zero() {
            let (remainder_leading_zeros, remainder_bit_length) = leading_zeros(&remainder);
            let (divisor_leading_zeros, divisor_bit_length) = leading_zeros(&divisor);

            let len_a = (remainder_bit_length - remainder_leading_zeros) as u32;
            let len_b = (divisor_bit_length - divisor_leading_zeros) as u32;

            if len_b > len_a {
                break;
            }

            // Выравниваем делитель с текущим остатком
            divisor <<= len_a - len_b;
            remainder ^= &divisor; // Вычитание в GF(2) также является XOR
            divisor >>= len_a - len_b; // Восстанавливаем позицию делителя
        }
        remainder.to_u64().unwrap()
    }
    //00001001 x^8 + x^4 + x^3 + x + 1

    // println!("{:04b}", 0b10000000 ^ (0b10011 << 3));
    // println!("{:04b}", 0b110000 ^ (0b10011 << 1));  Пример функции module redaction, которая берёт остаток от деления двочиного числа на двоичное число
    // println!("{:04b}", 0b10110 ^ 0b10011);
}
