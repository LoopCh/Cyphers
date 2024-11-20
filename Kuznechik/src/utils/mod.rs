pub mod gfield;
pub mod consts;

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
