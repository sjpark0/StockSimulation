use crate::backtester::Backtester;
use crate::types::{Assets_Hashmap, CapitalReturns};
use std::collections::HashMap;

pub struct RebalancePortfolioHashmap{
    initial_capital : f64,
    assets : Assets_Hashmap,
    fee_rate : f64,
    pub threshold: f64,    
    price_histories : HashMap<String, Vec::<f64>>,
}

impl RebalancePortfolioHashmap{    
    pub fn new(initial_capital: f64, tickers_fraction : &[(String, f64)], fee_rate : f64, threshold: f64, price_histories : HashMap<String, Vec::<f64>>) -> Self{
        let mut assets = Assets_Hashmap(HashMap::new());
        let mut fractions = 1.0;
        for (t, f) in tickers_fraction.iter(){
            assets.insert(t.to_string(), (0.0, *f));
            fractions -= *f;
        }
        assets.insert("CASH".to_string(), (0.0, fractions));
        Self { initial_capital : initial_capital, assets : assets, fee_rate : fee_rate, threshold : threshold, price_histories : price_histories}

    }
    fn rebalance(&mut self, current_prices: &[(String, f64)], total_value: f64){

        let mut total_stock_value = 0.0;
        for (t, p) in current_prices.iter(){
            if let Some((qty, f)) = self.assets.get_mut(t){
                let pre_qty = *qty;
                *qty = (total_value * *f / p).floor();
                let fee = (*qty - pre_qty).abs() * self.fee_rate * 0.01;
                total_stock_value += *qty * p - fee;
            }
        }
        if let Some((val, f)) = self.assets.get_mut("CASH"){
            *val = total_value - total_stock_value;
        }
    }
    fn process_price(&mut self, current_prices: &[(String, f64)]){
        let mut total_val = 0.0;
        let mut current_val = Vec::new();
        for (t, p) in current_prices.iter(){
            if let Some((qty, _)) = self.assets.get(t){
                current_val.push(*qty * p);
                total_val += *qty * p;
            }
        }
        if let Some((val, _)) = self.assets.get("CASH"){
            current_val.push(*val);
            total_val += *val;
        }
        let mut total_ratio = 0.0;
        for (t, p) in current_prices.iter(){
            if let Some((qty, fration)) = self.assets.get(t){
                total_ratio += (*qty * p / total_val - fration).abs();
            }
        }
        if let Some((val, fration)) = self.assets.get("CASH"){
            total_ratio += (*val / total_val - fration).abs();
        }
        
        if total_ratio >= self.threshold{
            self.rebalance(current_prices, total_val);
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

impl Backtester for RebalancePortfolioHashmap{
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
        let mut current_prices = vec![("".to_string(), 0.0);self.price_histories.len()];

        for i in start..=end{
            for (id, (k, v)) in self.price_histories.iter().enumerate(){
                current_prices[id] = (k.to_string(), v[i]);
            }            
            self.process_price(&current_prices);
        }

        for (id, (k, v)) in self.price_histories.iter().enumerate(){
            current_prices[id] = (k.to_string(), v[end]);
        }            
        self.get_total_rate(&current_prices)
    }
        
    fn initial_investment(&mut self){
        for (val1, _) in self.assets.values_mut(){
            *val1 = 0.0;            
        }        
        if let Some((val, _)) = self.assets.get_mut("CASH"){
            *val = self.initial_capital;
        }
    }
    
}
