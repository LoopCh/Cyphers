#[cfg(test)]
mod tests {
    use traits::block_encryptor::BlockEncryptor;
    use crate::{discard_every_eighth_bit, DES};
    use super::*;

    #[test]
    fn test_discard_every_eighth_bit() {
        let input1: [u8; 8] = [0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111];
        let expected1: u64 = 0b1111111011111110111111101111111011111110111111101111111011111110; // 0xFFFFFFFFFFFFFFFE
        assert_eq!(discard_every_eighth_bit(&input1), expected1);
    }

    #[test]
    fn test_des() {
        let key: [u8;8] = [0x12,0x23,0x34,0x56,0x78,0x90,0xff,0xee];
        let input_data:[u8;8] = [0xff,0xee,0x89,0x90,0x14,0x78,0xff,0x77];

        let DES = DES::new(&key);

        let encrypt_block = DES.encrypt_block(input_data);
        let decrypt_block = DES.decrypt_block(encrypt_block);

        assert_eq!(input_data, decrypt_block);
    }
}