#[cfg(test)]
mod tests {
    use crate::utils::gfield::GaluaField2;
    use num_bigint::BigInt;
    use crate::*;

    #[test]
    fn test_mul() {
        let a = BigInt::from(0b11111111);
        let b = BigInt::from(0b11111111);
        let gfield = GaluaField2::new(&[1, 0, 0, 0, 1, 1, 0, 1, 1]);

        let result = gfield.multiply_polynomials_by_irreducible_poly(&a, &b);

        assert_eq!(result, 0b10011)
    }

    use super::*;
    #[test]
    fn test_shift_rows() {
        let mut state = [1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4];
        let expected = [1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4];
        let original = [1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4];
        shift_rows(&mut state);
        assert_eq!(state, expected);
        inv_shift_rows(&mut state);
        assert_eq!(state, original);
    }
    #[test]
    fn test_byte_sub() {
        let original = [1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4];
        let mut state = [1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4];
        sub_bytes(&mut state);
        inv_sub_bytes(&mut state);
        assert_eq!(state, original);
    }

    #[test]
    fn test_gal_mul() {
        let gfield = GaluaField2::new(&[1, 0, 0, 0, 1, 1, 0, 1, 1]);

        assert_eq!(gfield.multiply_polynomials_by_irreducible_poly(&2, &2), 4);
        assert_eq!(gfield.multiply_polynomials_by_irreducible_poly(&6, &3), 10);
        assert_eq!(gfield.multiply_polynomials_by_irreducible_poly(&7, &12), 36);
        assert_eq!(gfield.multiply_polynomials_by_irreducible_poly(&12, &12), 80);
    }

    #[test]
    fn test_col_shift() {
        let mut state = [1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4];
        let original = [1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4];
        mix_cols(&mut state);
        inv_mix_cols(&mut state);
        assert_eq!(state, original);
    }
    #[test]
    fn test_aes() {
        let key = [0x00,0x01,0x02,0x03,0x04,0x05,0x06,0x07,0x08,0x09,0x0a,0x0b,0x0c,0x0d,0x0e,0x0f];
        let block = [0x00,0x11,0x22,0x33,0x44,0x55,0x66,0x77,0x88,0x99,0xaa,0xbb,0xcc,0xdd,0xee,0xff];

        let aes = AES::new(key);
        let encrypted_block = aes.encrypt_block(block);
        let decrypted_block = aes.decrypt_block(encrypted_block);
        assert_eq!(block,decrypted_block)
    }
}
