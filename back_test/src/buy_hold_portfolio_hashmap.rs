use crate::backtester::Backtester;
use crate::types::{Assets_Hashmap, CapitalReturns, StockPrices};
use std::collections::HashMap;

pub struct BuyHoldPortfolioHashmap{
    initial_capital : f64,
    assets : Assets_Hashmap,
    fee_rate : f64,
    price_histories : HashMap<String, StockPrices>,
}

impl BuyHoldPortfolioHashmap{    
    pub fn new(initial_capital: f64, fee_rate: f64, tickers_fraction : &[(String, f64)], price_histories : HashMap<String, StockPrices>) -> Self{
        let mut assets = Assets_Hashmap(HashMap::new());
        let mut fractions = 1.0;
        for (t, f) in tickers_fraction.iter(){
            assets.insert(t.to_string(), (0.0, *f));
            fractions -= *f;
        }
        assets.insert("CASH".to_string(), (0.0, fractions));
        Self { initial_capital : initial_capital, assets : assets, fee_rate : fee_rate, price_histories : price_histories}
    }
    fn process_price(&mut self, date_idx : usize){
        let mut total_stock = 0.0;
        for (t, p) in self.price_histories.iter(){
            if let Some((qty, f)) = self.assets.get_mut(t){
                *qty = (self.initial_capital * *f / p[date_idx]).floor();
                total_stock += *qty * p[date_idx];
            }
        }
        if let Some((val, f)) = self.assets.get_mut("CASH"){
            *val = self.initial_capital - total_stock * (1.0 + 0.01 * self.fee_rate);
        }
    }
    
}

impl Backtester for BuyHoldPortfolioHashmap{
    fn rolling_return(&mut self, duration : usize) -> CapitalReturns{    
        let length = self.price_histories.values().next().unwrap_or(&StockPrices(Vec::new())).len();
        CapitalReturns((0..length).map(|idx| if idx < duration { None } else { Some(self.process_backtester(idx - duration, idx)) }).collect())            
    }

    fn process_backtester(&mut self, start : usize, end : usize) -> (f64, f64){
        let mut local_maximum: f64 = 0.0;
        let mut mdd: f64 = 0.0;

        self.initial_investment();
        self.process_price(start);
        for i in start..end{
            let total_val = self.price_histories.iter().fold(self.assets.get("CASH").unwrap().0, |acc: f64, (ticker, p)| acc + self.assets.get(ticker).unwrap().0 * p[i]);
            local_maximum = local_maximum.max(total_val);
            mdd = mdd.max(1.0 - total_val / local_maximum);        
        }
        let total_val = self.price_histories.iter().fold(self.assets.get("CASH").unwrap().0, |acc: f64, (ticker, p)| acc + self.assets.get(ticker).unwrap().0 * p[end]);
            
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
