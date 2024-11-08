use std::ops::BitXor;
use num_bigint::{BigInt, ToBigInt};
use num_traits::{Pow, ToPrimitive, Zero, One};

pub struct GaluaField2 {
    elements: Vec<u8>,
    irreducible_polynomial: u64,
}

impl GaluaField2{
    pub fn new<const DEGREE: usize, const DEGREE_ELEMENT: usize>(irreducible_poly: &[u8; DEGREE], generation: &[u8; DEGREE_ELEMENT]) -> GaluaField2 {
        GaluaField2 {
            elements: Self::get_elements(irreducible_poly, generation),
            irreducible_polynomial: bits_to_byte::<DEGREE>(&irreducible_poly),
        }
    }

    pub fn show_elements(&self) {
        for (i, el) in self.elements.iter().enumerate() {
            println!("i: {i}, el: {:08b}", el);
        }
    }

    fn get_elements<const DEGREE: usize, const DEGREE_ELEMENT: usize>(irreducible_poly: &[u8; DEGREE], generation: &[u8; DEGREE_ELEMENT]) -> Vec<u8> {

        let poly_number = bits_to_byte::<DEGREE>(&irreducible_poly);
        println!("poly_number: {:b}", poly_number);
        let mut galua_field = Vec::new();

        let mut generation: u64 = bits_to_byte::<DEGREE_ELEMENT>(generation);
        let mut temprory = BigInt::from(1);

        println!("generation: {:b}", generation);
        println!("generation: {:b}", multiply_polynomials(&BigInt::from(generation), &BigInt::from(generation)).to_u64().unwrap());

        for _ in 0..2_usize.pow(DEGREE_ELEMENT as u32) {
            galua_field.push(get_binary_remainder(temprory.clone(), BigInt::from(poly_number)) as u8);
            temprory = multiply_polynomials(&temprory, &generation.to_bigint().unwrap());
            //println!("{:b}", galua_field[i]);
        }
        galua_field
    }

    pub fn mul(a: usize, b: usize) {
        println!()
    }
}

pub fn multiply_polynomials(a: &BigInt, b: &BigInt) -> BigInt {
    let mut result = BigInt::zero();
    let mut a_shifted = a.clone();

    // Проходим по каждому биту b
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

    result
}

pub fn get_binary_remainder(mut remainder: BigInt, mut divisor: BigInt) -> u64 {
    while &divisor != &BigInt::zero() {
        let temp = leading_zeros(&remainder);
        let temp2 = leading_zeros(&divisor);
        let len_a = (temp.1 - temp.0) as u32;
        let len_b = (temp2.1 - temp2.0) as u32;

        if len_b > len_a {
            break;
        }

        divisor = &divisor << (len_a - len_b);
        remainder = &remainder ^ &divisor;
        divisor = &divisor >> (len_a - len_b);
    }
    remainder.to_u64().unwrap()
}


fn leading_zeros(value: &BigInt) -> (usize, usize) {
    if value.is_zero() {
        return (0,0); // Если значение 0, можно считать, что ведущих нулей нет
    }

    // Преобразуем число в двоичную строку
    let binary_str = value.to_str_radix(2);

    // Подсчитаем количество ведущих нулей
    (binary_str.chars().take_while(|&c| c == '0').count(), binary_str.len())
}




fn bits_to_byte<const COUNT_ELEMENTS: usize>(bits: &[u8; COUNT_ELEMENTS]) -> u64 {
    // Убедимся, что COUNT_ELEMENTS не превышает 64, так как результат — u64
    assert!(COUNT_ELEMENTS <= 64, "COUNT_ELEMENTS должен быть не больше 64");

    // Преобразуем массив `bits` в одно 64-битное число
    bits.iter()
        .enumerate()
        .fold(0u64, |acc, (i, &bit)| acc | ((bit as u64 & 1) << (COUNT_ELEMENTS - 1 - i)))
}

//00001001 x^8 + x^4 + x^3 + x + 1