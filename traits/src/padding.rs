use std::iter::repeat;

pub trait Padding<const BLOCK_SIZE: usize> {
    fn pud(&self, data: &[u8]) -> Vec<u8>;
    fn unpad(&self, data: &[u8]) -> Vec<u8>;
}

pub struct PKCS7;

impl<const BLOCK_SIZE: usize> Padding<BLOCK_SIZE> for PKCS7 {
    fn pud(&self, data: &[u8]) -> Vec<u8> {

    let padding_len = BLOCK_SIZE - (data.len() % BLOCK_SIZE);
    let mut padded = Vec::with_capacity(data.len() + padding_len);
    padded.extend_from_slice(data);
    padded.extend(repeat(padding_len as u8).take(padding_len));

    padded
    }
    fn unpad(&self, data: &[u8]) -> Vec<u8> {
        let padding_len = data[data.len() - 1] as usize;

        data[..data.len()-padding_len].to_vec()
    }
}