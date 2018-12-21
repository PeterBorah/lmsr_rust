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

    pub fn trade(&mut self, outcome_id: usize, shares: f64) {
        self.outstanding_shares[outcome_id] += shares;
    }
}

pub struct Portfolio {
    pub outcome_shares: Vec<f64>,
    pub collateral: f64,
}

pub struct Market {
    pub market_maker: MarketMaker,
    pub portfolios: HashMap<String, Portfolio>,
    pub num_outcomes: usize,
}

impl Market {
    pub fn new(b: f64, num_outcomes: usize) -> Market {
        let market_maker = MarketMaker::new(b, num_outcomes);
        let portfolios = HashMap::new();

        Market { market_maker, portfolios, num_outcomes }
    }

    pub fn add_collateral(&mut self, address: String, amount: f64) {
        let portfolio = self.portfolios.entry(address).or_insert(Portfolio {
            outcome_shares: vec![0.0; self.num_outcomes],
            collateral: 0.0
        });

        portfolio.collateral += amount;
    }

    pub fn trade(&mut self, address: String, outcome_id: usize, shares: f64) {
        match self.portfolios.get_mut(&address) {
            None => return,
            Some(portfolio) => {
                let cost = self.market_maker.cost_to_trade(outcome_id, shares);
                if portfolio.collateral >= cost {
                    portfolio.outcome_shares[outcome_id] += shares;
                    portfolio.collateral -= cost;
                    self.market_maker.trade(outcome_id, shares);
                } else {
                    return;
                }
            }
        }
    }

    pub fn buy_with_max_price(&mut self, address: String, outcome_id: usize, shares: f64, max_price: f64) {
        if shares < 0.0 { return };

        let shares_to_max = self.market_maker.shares_to_set_price(outcome_id, max_price);
        if shares_to_max < shares {
            self.trade(address, outcome_id, shares_to_max);
        } else {
            self.trade(address, outcome_id, shares);
        }
    }
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
        let mut market_maker = MarketMaker::new(100.0, 2);
        let trade_size = 10.0;

        market_maker.trade(0, trade_size);

        let result = market_maker.outstanding_shares[0];

        assert_within_epsilon(result, trade_size);
    }

    #[test]
    fn market_can_be_created() {
        let market = Market::new(100.0, 2);

        assert_eq!(market.market_maker.b, 100.0);
    }

    #[test]
    fn market_allows_adding_collateral() {
        let mut market = Market::new(100.0, 2);
        let address = "0x6891Ac4E2EF3dA9bc88C96fEDbC9eA4d6D88F768";

        market.add_collateral(String::from(address), 100.0);

        let result = market.portfolios[&String::from(address)].collateral;

        assert_eq!(result, 100.0);
    }

    #[test]
    fn market_allows_trades() {
        let mut market = Market::new(100.0, 2);
        let address = "0x6891Ac4E2EF3dA9bc88C96fEDbC9eA4d6D88F768";
        let shares = 10.0;

        market.add_collateral(String::from(address), 20.0);
        market.trade(String::from(address), 1, shares);

        let portfolio = &market.portfolios[&String::from(address)];
        let final_shares = portfolio.outcome_shares[1];
        assert_eq!(final_shares, shares);

        let final_collateral = portfolio.collateral;
        let expected_collateral = 20.0 - 5.124947;
        assert_within_epsilon(final_collateral, expected_collateral);
    }

    #[test]
    fn insufficient_collateral_noops() {
        let mut market = Market::new(100.0, 2);
        let address = "0x6891Ac4E2EF3dA9bc88C96fEDbC9eA4d6D88F768";
        let shares = 10.0;

        market.add_collateral(String::from(address), 4.0);
        market.trade(String::from(address), 1, shares);

        let portfolio = &market.portfolios[&String::from(address)];
        let final_shares = portfolio.outcome_shares[1];
        assert_eq!(final_shares, 0.0);

        let final_collateral = portfolio.collateral;
        assert_within_epsilon(final_collateral, 4.0);
    }

    #[test]
    fn buy_with_max_price_works() {
        let mut market = Market::new(100.0, 2);
        let address = "0x6891Ac4E2EF3dA9bc88C96fEDbC9eA4d6D88F768";
        let shares = 1000.0;
        let max_price = 0.6;
        let outcome_id = 0;

        market.add_collateral(String::from(address), 10000.0);
        market.buy_with_max_price(String::from(address), outcome_id, shares, max_price);

        let price = market.market_maker.price(outcome_id);
        assert_within_epsilon(price, max_price);

        let portfolio = &market.portfolios[&String::from(address)];
        let shares_bought = portfolio.outcome_shares[outcome_id];
        assert!(shares_bought < shares);
    }

    #[test]
    fn buy_with_max_price_works_if_max_price_not_hit() {
        let mut market = Market::new(100.0, 2);
        let address = "0x6891Ac4E2EF3dA9bc88C96fEDbC9eA4d6D88F768";
        let shares = 1.0;
        let max_price = 0.9;
        let outcome_id = 0;

        market.add_collateral(String::from(address), 10000.0);
        market.buy_with_max_price(String::from(address), outcome_id, shares, max_price);

        let price = market.market_maker.price(outcome_id);
        assert!(price < max_price);

        let portfolio = &market.portfolios[&String::from(address)];
        let shares_bought = portfolio.outcome_shares[outcome_id];
        assert_within_epsilon(shares_bought, shares);
    }
}
