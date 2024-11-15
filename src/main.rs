use magma::magma_block_encryptor::MagmaBlockEncryptor;
use DES;
use traits::BlockEncryptor::BlockEncryptor;


fn main() {
    let key = [
        0xff, 0xee, 0xdd, 0xcc, 0xbb, 0xaa, 0x99, 0x88,
        0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11, 0x00,
        0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7,
        0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff,
    ];
    let magma = MagmaBlockEncryptor::new(&key);
    let block = [0xfe,0xdc,0xba,0x98,0x76,0x54,0x32,0x10];

    let decrypt_block = magma.encrypt_block(block);
    let encrypted_block = magma.decrypt_block(decrypt_block);

}

fn test_cypher<E: BlockEncryptor<BLOCK_SIZE, KEY_SIZE>, const BLOCK_SIZE: usize, const KEY_SIZE: usize>(encryptor: &E, block: [u8; BLOCK_SIZE]) {
    dbg!(encryptor.encrypt_block(block));
}

