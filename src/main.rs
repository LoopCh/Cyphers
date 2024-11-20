use magma::magma_block_encryptor::MagmaBlockEncryptor;
use DES;
use traits::block_encryptor::BlockEncryptor;
use AES;
use traits::encryption_mode::{CBC, CFB, CTR, EBC, OFB};
use traits::padding::PKCS7;

fn main() {
    let key = [
        0x88,0x99,0xaa,0xbb,0xcc,0xdd,0xee,0xff,0x00,0x11,0x22,0x33,0x44,0x55,0x66,0x77,
        0xfe,0xdc,0xba,0x98,0x76,0x54,0x32,0x10,0x01,0x23,0x45,0x67,0x89,0xab,0xcd,0xef
    ];
    let block = [0x11,0x22,0x33,0x44,0x55,0x66,0x77,0x00,0xff,0xee,0xdd,0xcc,0xbb,0xaa,0x99,0x88];
    let iv = [0x11,0x22,0x33,0x44,0x55,0x66,0x77,0x00,0xff,0xee,0xdd,0xcc,0xbb,0xaa,0x99,0x88];
    let kuznechik = Kuznechik::Kuznechik::new(key);

    let encrypted_block = kuznechik.encrypt_block(block);
    let ebc_encrypted_block = kuznechik.CTR_encrypt(&"My text for EBC encryption mode".as_bytes(), &PKCS7, iv);
    println!("text_bytes: {:x?}", "My text for EBC encryption mode".as_bytes());
    println!("encrypted_block: {:x?}", ebc_encrypted_block);

    let decrypted_block = kuznechik.CTR_decrypt(&ebc_encrypted_block, &PKCS7, iv);
    println!("decrypted_block: {:x?}", decrypted_block);

    // --------------------- MAGMA -------------------------
    // let key = [
    //     0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
    //     0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00,
    //     0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
    //     0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    // ];
    //
    // // let block = [0xfe,0xdc,0xba,0x98,0x76,0x54,0x32,0x10];
    // // print!("block: ");
    // // for byte in &block {
    // //     print!("{:02x} ", byte);
    // // }
    // // println!();
    //
    // let magma = MagmaBlockEncryptor::new(&key);
    //
    // let encrypted_block = [0x1c,0x72,0x0b,0xf9,0xe8,0x7e,0xd0,0x54];
    // // print!("encrypted_block: ");
    // // for byte in &encrypted_block {
    // //     print!("{:02x} ", byte);
    // // }
    // // println!();
    //
    // let decrypted_block = magma.decrypt_block(encrypted_block);
    // print!("decrypted_block: ");
    // for byte in &decrypted_block {
    //     print!("{:02x} ", byte);
    // }
    // println!();
    //
    // println!("text:{}", String::from_utf8_lossy(&decrypted_block).to_string())

    //--------------------- DES -------------------------------

    // let key: [u8;8] = [0x12,0x23,0x34,0x56,0x78,0x90,0xff,0xee];
    // let block:[u8;8] = [0xff,0xee,0x89,0x90,0x14,0x78,0xff,0x77];
    // print!("block: ");
    // for byte in &block {
    //     print!("{:02x} ", byte);
    // }
    // println!();
    //
    // let DES = DES::DES::new(&key);
    //
    // let encrypted_block = DES.encrypt_block(block);
    // print!("encrypted_block: ");
    // for byte in &encrypted_block {
    //     print!("{:02x} ", byte);
    // }
    // println!();
    // let decrypted_block = DES.decrypt_block(encrypted_block);
    // print!("decrypted_block: ");
    // for byte in &decrypted_block {
    //     print!("{:02x} ", byte);
    // }
    // println!();

    // --------------------- AES ---------------------
    // let key = [0x00,0x01,0x02,0x03,0x04,0x05,0x06,0x07,0x08,0x09,0x0a,0x0b,0x0c,0x0d,0x0e,0x0f]; // [0x2b,0x7e,0x15,0x16,0x28,0xae,0xd2,0xa6,0xab,0xf7,0x15,0x88,0x09,0xcf,0x4f,0x3c];
    // let block = [0x00,0x11,0x22,0x33,0x44,0x55,0x66,0x77,0x88,0x99,0xaa,0xbb,0xcc,0xdd,0xee,0xff];
    // print!("block: ");
    // for byte in &block {
    //     print!("{:02x} ", byte);
    // }
    // println!();
    //
    // let aes = AES::AES::new(key);
    //
    // let encrypted_block = aes.encrypt_block(block);
    // print!("encrypted_block: ");
    // for byte in &encrypted_block {
    //     print!("{:02x} ", byte);
    // }
    // println!();
    // let decrypted_block = aes.decrypt_block(encrypted_block);
    // print!("decrypted_block: ");
    // for byte in &decrypted_block {
    //     print!("{:02x} ", byte);
    // }
    // println!();
}


