use num_bigint::BigInt;
use num_traits::Zero;

pub mod consts;
pub mod gfield;
pub mod tests;

fn bits_to_byte<const COUNT_ELEMENTS: usize>(bits: &[u8; COUNT_ELEMENTS]) -> u64 {
    // Убедимся, что COUNT_ELEMENTS не превышает 64, так как результат — u64
    assert!(
        COUNT_ELEMENTS <= 64,
        "COUNT_ELEMENTS должен быть не больше 64"
    );

    // Преобразуем массив `bits` в одно 64-битное число
    bits.iter().enumerate().fold(0u64, |acc, (i, &bit)| {
        acc | ((bit as u64 & 1) << (COUNT_ELEMENTS - 1 - i))
    })
}

fn leading_zeros(value: &BigInt) -> (usize, usize) {
    if value.is_zero() {
        return (0, 0); // Если значение 0, можно считать, что ведущих нулей нет
    }

    // Преобразуем число в двоичную строку
    let binary_str = value.to_str_radix(2);

    // Подсчитаем количество ведущих нулей
    (
        binary_str.chars().take_while(|&c| c == '0').count(),
        binary_str.len(),
    );

    let leading_zeros_count = binary_str.chars().take_while(|&c| c == '0').count();

    (leading_zeros_count, binary_str.len())
}
