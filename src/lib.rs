pub const MIN_BLOB_BASE_FEE: u128 = 1;
pub const CANCUN_FRACTION: u128 = 3_338_477;
pub const PRAGUE_FRACTION: u128 = 5_007_716;

/// Integer approximation of factor * e^(numerator / denominator), as specified in EIP-4844.
pub fn fake_exponential(factor: u128, numerator: u128, denominator: u128) -> u128 {
    let mut i: u128 = 1;
    let mut output: u128 = 0;
    let mut acc = factor * denominator;
    while acc > 0 {
        output += acc;
        acc = acc * numerator / (denominator * i);
        i += 1;
    }
    output / denominator
}

pub fn blob_base_fee(excess_blob_gas: u128, fraction: u128) -> u128 {
    fake_exponential(MIN_BLOB_BASE_FEE, excess_blob_gas, fraction)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimum_at_zero_excess() {
        assert_eq!(blob_base_fee(0, CANCUN_FRACTION), 1);
    }

    #[test]
    fn approximates_exponential() {
        // e^1 * 1e9 should be close to 2.718e9
        let v = fake_exponential(1_000_000_000, 1_000, 1_000) as f64;
        assert!((v / 1e9 - std::f64::consts::E).abs() < 1e-6);
    }

    #[test]
    fn prague_is_cheaper_for_same_excess() {
        let excess = 60_000_000;
        assert!(blob_base_fee(excess, PRAGUE_FRACTION) < blob_base_fee(excess, CANCUN_FRACTION));
    }

    #[test]
    fn monotonic() {
        let mut last = 0;
        for e in (0..100_000_000).step_by(5_000_000) {
            let f = blob_base_fee(e, CANCUN_FRACTION);
            assert!(f >= last);
            last = f;
        }
    }
}
