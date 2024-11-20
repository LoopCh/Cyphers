pub trait BlockEncryptor<const B: usize, const C: usize> {
    fn encrypt_block(&self, block: [u8; B]) -> [u8; C];
    fn decrypt_block(&self, block: [u8; C]) -> [u8; B]; // ЕСЛИ ЧТО ТО СЛОМАЕТСЯ, ТО ПОМЕНЯТЬ C И B МЕСТАМИ
}

