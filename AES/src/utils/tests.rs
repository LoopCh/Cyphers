#[cfg(test)]
mod tests {
    use num_bigint::BigInt;
    use crate::utils::gfield::{get_binary_remainder, multiply_polynomials};
    #[test]
    fn test_binary() {
        let remainder = BigInt::from(0b10000000u64);
        let divisor = BigInt::from(0b10011u64);
        let result = get_binary_remainder(remainder, divisor);
        assert_eq!(result, 0b1011);
    }

    #[test]
    fn test_mul() {
        let a = BigInt::from(0b11111111);
        let b = BigInt::from(0b11111111);

        let result = multiply_polynomials(&a, &b);
        let mod_result = get_binary_remainder(result, BigInt::from(0b100011011));

        assert_eq!(BigInt::from(mod_result), BigInt::from(0b10011))
    }
}