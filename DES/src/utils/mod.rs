pub mod permutations;
pub mod key_gen;
pub mod sboxes;

/// Сдвиги влево для генерации раундовых ключей
pub const LEFT_SHIFTS: [usize; 16] = [
    1, 1, 2, 2, 2, 2, 2, 2,
    1, 2, 2, 2, 2, 2, 2, 1
];
