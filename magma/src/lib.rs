pub mod magma_block_encryptor;
mod utils;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::magma_block_encryptor::MagmaBlockEncryptor;
    use traits::block_encryptor::BlockEncryptor;
    use crate::utils::{mod32add};
    use super::*;

    #[test]
    fn test_u32() {
        let bytes = [10,200,31,44];
        let test_number = u32::from_be_bytes(bytes);
        let mut temp_array = [0;8];

        for i in 0..8 {
            temp_array[i] = ((test_number >> (i * 4)) & 0xF) as u8;
        }
        let mut expected_number = 0;
        for i in 0..temp_array.len() {
            expected_number |= (temp_array[i] as u32) << (i * 4) ;
        }

        assert_eq!(test_number, expected_number);
    }

    #[test]
    fn test_mod32() {
        let a = 4293844428;
        let b = 1985229328;
        let result = mod32add(a, b);
        assert_eq!(1984106460, result)
    }

    #[test]
    fn test_encrypt_magma() {
        let key = [
            0xff, 0xee, 0xdd, 0xcc, 0xbb, 0xaa, 0x99, 0x88,
            0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11, 0x00,
            0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7,
            0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff,
        ];
        let magma = MagmaBlockEncryptor::new(&key);
        let block = [0xfe,0xdc,0xba,0x98,0x76,0x54,0x32, 0x10];

        let decrypt_block = magma.encrypt_block(block);

        let encrypted_block = magma.decrypt_block(decrypt_block);
        assert_eq!(block, encrypted_block);
    }

}
