use crate::backtester::Backtester;
use crate::types::Assets;
use std::collections::HashMap;

pub struct BuyHoldPortfolio{
    initial_capital : f64,
    assets : Assets,
    fee_rate : f64,
}

impl BuyHoldPortfolio{    
    pub fn new(initial_capital: f64, fee_rate: f64, tickers_fraction : &[(String, f64)]) -> Self{
        let mut assets = Assets(HashMap::new());
        let mut fractions = 1.0;
        for (t, f) in tickers_fraction.iter(){
            assets.insert(t.to_string(), (0.0, *f));
            fractions -= *f;
        }
        assets.insert("CASH".to_string(), (0.0, fractions));
        Self { initial_capital : initial_capital, assets : assets, fee_rate : fee_rate}
    }    
}

impl Backtester for BuyHoldPortfolio{
    fn process_backtester(&mut self, price_histories : &HashMap<String, Vec::<f64>>, start : usize, end : usize) -> (f64, f64){
        self.initial_investment();
        let mut start_prices = Vec::new();
        let mut end_prices = Vec::new();
        for (k, v) in price_histories.iter(){
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