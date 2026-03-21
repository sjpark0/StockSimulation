use crate::backtester::Backtester;
use crate::types::{Assets_Hashmap, CapitalReturns};
use std::collections::HashMap;

pub struct BuyHoldPortfolioHashmap{
    initial_capital : f64,
    assets : Assets_Hashmap,
    fee_rate : f64,
    price_histories : HashMap<String, Vec::<f64>>,
}

impl BuyHoldPortfolioHashmap{    
    pub fn new(initial_capital: f64, fee_rate: f64, tickers_fraction : &[(String, f64)], price_histories : HashMap<String, Vec::<f64>>) -> Self{
        let mut assets = Assets_Hashmap(HashMap::new());
        let mut fractions = 1.0;
        for (t, f) in tickers_fraction.iter(){
            assets.insert(t.to_string(), (0.0, *f));
            fractions -= *f;
        }
        assets.insert("CASH".to_string(), (0.0, fractions));
        Self { initial_capital : initial_capital, assets : assets, fee_rate : fee_rate, price_histories : price_histories}
    }
    fn process_price(&mut self, current_prices: &[(String, f64)]){
        let mut total_stock = 0.0;
        for (t, p) in current_prices.iter(){
            if let Some((qty, f)) = self.assets.get_mut(t){
                *qty = (self.initial_capital * *f / p).floor();
                total_stock += *qty * p;
            }
        }
        if let Some((val, f)) = self.assets.get_mut("CASH"){
            *val = self.initial_capital - total_stock * (1.0 + 0.01 * self.fee_rate);
        }
    }
    
    fn get_total_rate(&self, current_prices : &[(String, f64)]) -> (f64, f64) {
        let mut total_val = 0.0;

        for (t, p) in current_prices.iter(){
            if let Some((qty, _)) = self.assets.get(t){
                total_val += *qty * p;
            }
        }
        if let Some((val, _)) = self.assets.get("CASH"){
            total_val += *val;
        }

        (total_val, total_val / self.initial_capital)
    }    
}

impl Backtester for BuyHoldPortfolioHashmap{
    fn rolling_return(&mut self, duration : usize) -> CapitalReturns{    
        let mut res_vec: CapitalReturns = CapitalReturns(Vec::new());        
        for i in 0..self.price_histories.values().next().unwrap_or(&Vec::new()).len(){
            if i < duration{
                res_vec.push(None);
            }
            else{
                let (cap, _) = self.process_backtester(i - duration, i);
                res_vec.push(Some(cap));
            }
        }
        res_vec
    }

    fn process_backtester(&mut self, start : usize, end : usize) -> (f64, f64){
        self.initial_investment();
        let mut start_prices = Vec::new();
        let mut end_prices = Vec::new();
        for (k, v) in self.price_histories.iter(){
            start_prices.push((k.to_string(), v[start]));
            end_prices.push((k.to_string(), v[end]));
        }
        self.process_price(&start_prices);
        self.get_total_rate(&end_prices)
    }
    
    fn initial_investment(&mut self){
        for (val1, _) in self.assets.values_mut(){
            *val1 = 0.0;            
        }        
    }
}
