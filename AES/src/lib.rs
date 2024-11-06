mod utils;
use traits::BlockEncryptor::BlockEncryptor;

struct AES {
    key: [[u8;4];4]
}

impl BlockEncryptor<16,16> for AES {
    fn encrypt_block(&self, block: [u8; 16]) -> [u8; 16] {
        todo!()
    }
    fn decrypt_block(&self, block: [u8; 16]) -> [u8; 16] {
        todo!()
    }
}



