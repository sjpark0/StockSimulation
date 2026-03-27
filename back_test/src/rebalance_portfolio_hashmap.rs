use crate::backtester::Backtester;
use crate::types::{AssetsHashmap, CapitalReturns, StockPrices};
use std::collections::HashMap;

pub struct RebalancePortfolioHashmap{
    initial_capital : f64,
    assets : AssetsHashmap,
    fee_rate : f64,
    pub threshold: f64,    
    price_histories : HashMap<String, StockPrices>,
}

impl RebalancePortfolioHashmap{    
    pub fn new(initial_capital: f64, tickers_fraction : &[(String, f64)], fee_rate : f64, threshold: f64, price_histories : HashMap<String, StockPrices>) -> Self{
        let mut assets = AssetsHashmap(HashMap::new());
        let mut fractions = 1.0;
        for (t, f) in tickers_fraction.iter(){
            assets.insert(t.to_string(), (0.0, *f));
            fractions -= *f;
        }
        assets.insert("CASH".to_string(), (0.0, fractions));
        Self { initial_capital : initial_capital, assets : assets, fee_rate : fee_rate, threshold : threshold, price_histories : price_histories}

    }
    fn rebalance(&mut self, date_idx : usize, total_value: f64){
        let mut total_stock_value = 0.0;
        for (t, p) in self.price_histories.iter(){
            if let Some((qty, f)) = self.assets.get_mut(t){
                let pre_qty = *qty;
                *qty = (total_value * *f / p[date_idx]).floor();
                let fee = (*qty - pre_qty).abs() * self.fee_rate * 0.01;
                total_stock_value += *qty * p[date_idx] + fee;
            }
        }
        if let Some((val, _)) = self.assets.get_mut("CASH"){
            *val = total_value - total_stock_value;
        }
    }
    fn process_price(&mut self, date_idx : usize) -> f64{        
        let total_val = self.price_histories.iter().fold(self.assets.get("CASH").unwrap().0, |acc: f64, (ticker, p)| acc + self.assets.get(ticker).unwrap().0 * p[date_idx]);        
        let total_ratio = self.price_histories.iter().fold(
            { 
                let init_val = self.assets.get("CASH").unwrap(); 
                (init_val.0 / total_val - init_val.1).abs() 
            }, 
            |acc: f64, (ticker, p)| {
                let current_val = self.assets.get(ticker).unwrap();
                acc + (current_val.0 * p[date_idx] / total_val - current_val.1).abs() 
            });                        
        
        if total_ratio >= self.threshold{
            self.rebalance(date_idx, total_val);
        }
        total_val
    }
    
    
}

impl Backtester for RebalancePortfolioHashmap{
    fn rolling_return(&mut self, duration : usize) -> CapitalReturns{    
        let length = self.price_histories.values().next().unwrap_or(&StockPrices(Vec::new())).len();
        CapitalReturns((0..length).map(|idx| if idx < duration { None } else { Some(self.process_backtester(idx - duration, idx)) }).collect())            
    }

    fn process_backtester(&mut self, start : usize, end : usize) -> (f64, f64){
        let mut local_maximum: f64 = 0.0;
        let mut mdd: f64 = 0.0;
        let mut total_val;
        self.initial_investment();
        for i in start..=end{
            total_val = self.process_price(i);
            local_maximum = total_val.max(local_maximum);
            mdd = mdd.max(1.0 - total_val / local_maximum);         
        }
        
        total_val = self.price_histories.iter().fold(self.assets.get("CASH").unwrap().0, |acc: f64, (ticker, p)| acc + self.assets.get(ticker).unwrap().0 * p[end]);
        (total_val, mdd)
        
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
