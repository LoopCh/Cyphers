

pub fn xor<const BLOCK_SIZE: usize, const CIPHER_BLOCK_SIZE: usize>(a: &[u8; BLOCK_SIZE], b:&[u8; CIPHER_BLOCK_SIZE]) -> [u8; BLOCK_SIZE]{
    let mut result = [0u8; BLOCK_SIZE];

    for i in 0..BLOCK_SIZE {
        result[i] = a[i] ^ b[i]
    };

    result
}

pub fn convert_array<const TEXT_BLOCK_SIZE: usize, const CIPHER_BLOCK_SIZE: usize>(value: [u8; CIPHER_BLOCK_SIZE]) -> [u8; TEXT_BLOCK_SIZE] {
    let mut result = [0u8; TEXT_BLOCK_SIZE]; // Создаем массив нужного размера
    result[..CIPHER_BLOCK_SIZE].copy_from_slice(&value); // Копируем данные
    result // Возвращаем новый массив
}