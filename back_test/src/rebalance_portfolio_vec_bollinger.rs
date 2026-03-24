use umya_spreadsheet::helper::date;

use crate::backtester::Backtester;
use crate::types::{Assets, CapitalReturns, StockPrices, BollingerBand};

pub struct RebalancePortfolioVecBollinger{
    initial_capital : f64,
    assets : Assets,
    fee_rate : f64,
    price_histories : Vec<StockPrices>,
    bollingerband_histories : Vec<BollingerBand>,
}

impl RebalancePortfolioVecBollinger{    
    pub fn new(initial_capital: f64, tickers_fraction : &[f64], fee_rate : f64, price_histories : Vec<StockPrices>, bollinger_badns : Vec<BollingerBand>) -> Self{
        let mut assets = Assets(Vec::new());
        let mut fractions = 1.0;
        for f in tickers_fraction.iter(){
            assets.push((0.0, *f));
            fractions -= *f;
        }
        assets.push((0.0, fractions));
        Self { initial_capital : initial_capital, assets : assets, fee_rate : fee_rate, price_histories : price_histories, bollingerband_histories : bollinger_badns}

    }
    fn rebalance(&mut self, date_idx : usize, total_value: f64){
        let mut total_stock_value = 0.0;
        let length = self.price_histories.len();
        for idx in 0..length{
            let pre_qty = self.assets[idx].0;
            self.assets[idx].0 = (total_value * self.assets[idx].1 / self.price_histories[idx][date_idx]).floor();
            let fee = (self.assets[idx].0 - pre_qty).abs() * self.fee_rate * 0.01;
            total_stock_value += self.assets[idx].0 * self.price_histories[idx][date_idx] + fee;            
        }
        self.assets[length].0 = total_value - total_stock_value;        
    }
    fn process_price(&mut self, date_idx : usize) -> f64{
        let length = self.price_histories.len();
        let total_val = (0..length).fold(self.assets[length].0, |acc: f64, idx| acc + self.assets[idx].0 * self.price_histories[idx][date_idx]);    
        
        if (0..length).fold(false, |acc, idx| acc | (self.price_histories[idx].is_not_range(&self.bollingerband_histories[idx], date_idx))){
            self.rebalance(date_idx, total_val);
        }            
        total_val
    }
    
}

impl Backtester for RebalancePortfolioVecBollinger{
    fn rolling_return(&mut self, duration : usize) -> CapitalReturns{    
        let length = self.price_histories[0].len();
        CapitalReturns((0..length).map(|idx| if idx < duration { None } else { Some(self.process_backtester(idx - duration, idx)) }).collect())            
    }

    fn process_backtester(&mut self, start : usize, end : usize) -> (f64, f64){
        let mut local_maximum: f64 = 0.0;
        let mut mdd: f64 = 0.0;
        let mut total_val = 0.0;

        self.initial_investment();
        for i in start..=end{
            total_val = self.process_price(i);
            local_maximum = total_val.max(local_maximum);
            mdd = mdd.max(1.0 - total_val / local_maximum);         
        }

        let length = self.price_histories.len();
        total_val = (0..length).fold(self.assets[length].0, |acc: f64, idx| acc + self.assets[idx].0 * self.price_histories[idx][end]);                
        (total_val, mdd)
    }
        
    fn initial_investment(&mut self){
        for (val1, _) in self.assets.iter_mut(){
            *val1 = 0.0;            
        }
        self.assets.last_mut().unwrap().0 = self.initial_capital;                
    }
}
