extern crate rug;
use std::f64::consts::E;

use rug::ops::Pow;

#[derive(Clone)]
pub struct Market {
    pub b: f64,
    pub outstanding_shares: Vec<f64>,
}

pub struct Trade {
    pub outcome_id: usize,
    pub shares: f64
}

pub fn cost_fn(market: Market) -> f64 {
    let b = market.b;
    let sum_of_exp = market.outstanding_shares.iter().fold(0_f64, |acc, x| acc + E.pow(x / b));

    b * sum_of_exp.ln()
}

pub fn cost_to_trade(market: Market, trade: Trade) -> f64 {
    let mut new_market = market.clone();
    new_market.outstanding_shares[trade.outcome_id] += trade.shares;

    cost_fn(new_market) - cost_fn(market)
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
        let outstanding_shares = vec!(10_f64, 0_f64);

        let market = Market { b, outstanding_shares };

        let result = cost_fn(market);
        let expected = 74.439666_f64;

        assert!((expected - result).abs() < 0.0001, "Got {}, expected {}", result, expected);
    }

    #[test]
    #[ignore]
    fn cost_fn_works_with_more_than_two_options() {
        // TODO
        unimplemented!();
    }

    #[test]
    fn cost_to_trade_works() {
        let b = 100_f64;
        let outstanding_shares = vec!(0_f64, 0_f64);

        let market = Market { b, outstanding_shares };
        let trade = Trade { outcome_id: 0, shares: 10.0 };

        let result = cost_to_trade(market, trade);
        let expected = 5.12;

        assert!((expected - result).abs() < 0.01, "Got {}, expected {}", result, expected);
    }
}
