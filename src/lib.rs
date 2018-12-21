extern crate rug;

use std::f64::consts::E;
use std::collections::HashMap;

use rug::ops::Pow;

#[derive(Clone)]
pub struct MarketMaker {
    pub b: f64,
    pub outstanding_shares: Vec<f64>,
}

impl MarketMaker {
    pub fn new(b: f64, num_outcomes: usize) -> MarketMaker {
        let outstanding_shares = vec![0_f64; num_outcomes];

        MarketMaker { b, outstanding_shares }
    }

    pub fn cost_fn(&self) -> f64 {
        self.b * self.sum_of_exp().ln()
    }

    // Calculates exp(q1/b) for each outcome and sums
    fn sum_of_exp(&self) -> f64 {
        self.outstanding_shares.iter().fold(0_f64, |acc, q| acc + E.pow(q / self.b))
    }

    pub fn cost_to_trade(&self, outcome_id: usize, shares: f64) -> f64 {
        let mut new_market_maker = self.clone();
        new_market_maker.outstanding_shares[outcome_id] += shares;

        new_market_maker.cost_fn() - self.cost_fn()
    }

    pub fn price(&self, outcome_id: usize) -> f64 {
        E.pow(self.outstanding_shares[outcome_id] / self.b) / self.sum_of_exp()
    }

    pub fn shares_to_set_price(&self, outcome_id: usize, new_price: f64) -> f64 {
        let current_price = self.price(outcome_id);
        self.b * ((new_price / current_price).ln() - ((1.0 - new_price) / (1.0 - current_price)).ln())
    }

    pub fn trade(self, outcome_id: usize, shares: f64) -> MarketMaker {
        let mut new_market_maker = self.clone();
        new_market_maker.outstanding_shares[outcome_id] += shares;

        new_market_maker
    }
}

pub struct Portfolio {
    pub outcome_tokens: Vec<f64>,
    pub collateral: f64,
}

pub struct Market {
    market_maker: MarketMaker,
    portfolios: HashMap<String, Portfolio>,
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::f64;

    use rug::ops::Pow;

    fn assert_within_epsilon(x: f64, y: f64) {
        assert!((x - y).abs() < 0.0001, "{} and {} aren't within epsilon", x, y);
    }

    #[test]
    fn e_works() {
        let result = f64::consts::E;
        let expected = 2.71828_f64;
        assert_within_epsilon(expected, result);
    }

    #[test]
    fn pow_works() {
        let e = f64::consts::E;
        let result = e.pow(2);
        let expected = 7.3890560_f64;
        assert_within_epsilon(expected, result);
    }

    #[test]
    fn cost_fn_works() {
        let b = 100_f64;
        let outstanding_shares = vec!(10_f64, 0_f64);

        let market_maker = MarketMaker { b, outstanding_shares };

        let result = market_maker.cost_fn();
        let expected = 74.439666_f64;

        assert_within_epsilon(expected, result);
    }

    #[test]
    #[ignore]
    fn cost_fn_works_with_more_than_two_options() {
        // TODO
        unimplemented!();
    }

    #[test]
    fn cost_to_trade_works() {
        let market_maker = MarketMaker::new(100.0, 2);

        let result = market_maker.cost_to_trade(0, 10.0);
        let expected = 5.124947;

        assert_within_epsilon(expected, result);
    }

    #[test]
    fn price_works() {
        let market_maker = MarketMaker::new(100.0, 2);

        let price_0 = market_maker.price(0);
        let price_1 = market_maker.price(1);

        assert_within_epsilon(price_0, price_1);
        assert_within_epsilon(price_0, 0.5_f64);
    }

    #[test]
    fn price_sums_to_1() {
        let b = 100_f64;
        let outstanding_shares = vec!(44_f64, 17_f64);

        let market_maker = MarketMaker { b, outstanding_shares };

        let price_0 = market_maker.price(0);
        let price_1 = market_maker.price(1);

        assert_within_epsilon(price_0 + price_1, 1.0);
    }

    #[test]
    fn shares_to_set_price_works() {
        let b = 100_f64;
        let shares_0 = 40_f64;
        let shares_1 = 12_f64;

        let outstanding_shares = vec!(shares_0, shares_1);
        let market_maker = MarketMaker { b, outstanding_shares };

        let target = 0.6_f64;
        let outcome_id = 1;

        let shares_to_buy = market_maker.shares_to_set_price(outcome_id, target);


        let outstanding_shares = vec!(shares_0, shares_1 + shares_to_buy);
        let market_maker = MarketMaker { b, outstanding_shares };

        let result = market_maker.price(outcome_id);

        assert_within_epsilon(target, result);
    }

    #[test]
    fn trade_works_on_mm() {
        let market_maker = MarketMaker::new(100.0, 2);
        let trade_size = 10.0;

        let market_maker = market_maker.trade(0, trade_size);

        let result = market_maker.outstanding_shares[0];

        assert_within_epsilon(result, trade_size);
    }
}
