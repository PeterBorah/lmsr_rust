extern crate rug;
use std::f64::consts::E;

use rug::ops::Pow;

pub struct Market {
    pub b: f64,
    pub outstanding_shares: (f64, f64),
}

pub fn cost_fn(market: Market) -> f64 {
    market.b * ((E.pow(market.outstanding_shares.0 / market.b) + E.pow(market.outstanding_shares.1 / market.b)).ln())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::f64;

    use rug::Float;
    use rug::ops::Pow;

    #[test]
    fn rug_works() {
        let f = Float::with_val(53, 1.5);
        let expected: f32 = 1.5;
        assert!((f - expected).abs() < 0.0001);
    }

    #[test]
    fn ln_works() {
        let f = Float::with_val(53, 1.5);
        let expected = 0.4055_f64;
        let result = f.ln();
        assert!((expected - result).abs() < 0.0001);
    }

    #[test]
    fn e_works() {
        let result = f64::consts::E;
        let expected = 2.71828_f64;
        assert!((expected - result).abs() < 0.0001);
    }

    #[test]
    fn pow_works() {
        let e = f64::consts::E;
        let result = e.pow(2);
        let expected = 7.3890560_f64;
        assert!((expected - result).abs() < 0.0001);
    }

    #[test]
    fn cost_fn_works() {
        let b = 100_f64;
        let outstanding_shares = (10_f64, 0_f64);

        let market = Market { b, outstanding_shares };

        let result = cost_fn(market);
        let expected = 74.439666_f64;

        assert!((expected - result).abs() < 0.0001);
    }
}
