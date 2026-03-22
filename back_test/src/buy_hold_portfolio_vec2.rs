use crate::backtester::Backtester;
use crate::types::{Assets, CapitalReturns};

pub struct BuyHoldPortfolioVec2{
    initial_capital : f64,
    assets : Assets,
    fee_rate : f64,
    price_histories : Vec<Vec::<f64>>,
}

impl BuyHoldPortfolioVec2{    
    pub fn new(initial_capital: f64, fee_rate: f64, tickers_fraction : &[f64], price_histories : Vec<Vec::<f64>>) -> Self{
        let mut assets = Assets(Vec::new());
        let mut fractions = 1.0;
        for f in tickers_fraction.iter(){
            assets.push((0.0, *f));
            fractions -= *f;
        }
        assets.push((0.0, fractions));
        Self { initial_capital : initial_capital, assets : assets, fee_rate : fee_rate, price_histories : price_histories}
    }
    fn process_price(&mut self, date_idx : usize){
        let mut total_stock = 0.0;
        for (idx, p) in self.price_histories.iter().enumerate(){
            self.assets[idx].0 = (self.initial_capital * self.assets[idx].1 / p[date_idx]).floor();            
            total_stock += self.assets[idx].0 * p[date_idx];
        }        
        self.assets[self.price_histories.len()].0 = self.initial_capital - total_stock * (1.0 + 0.01 * self.fee_rate);
    }
    
    fn get_total_rate(&self, date_idx : usize) -> (f64, f64) {
        let total_val = self.price_histories.iter().enumerate().fold(self.assets.last().unwrap().0, |acc: f64, (idx, p)| acc + self.assets[idx].0 * p[date_idx]);
        (total_val, total_val / self.initial_capital)        
    }    
}

impl Backtester for BuyHoldPortfolioVec2{
    fn rolling_return(&mut self, duration : usize) -> CapitalReturns{    
        let length = self.price_histories[0].len();
        CapitalReturns((0..length).map(|idx| if idx < duration { None } else { Some(self.process_backtester(idx - duration, idx).0) }).collect())            
    }
    fn process_backtester(&mut self, start : usize, end : usize) -> (f64, f64){
        self.initial_investment();        
        self.process_price(start);
        self.get_total_rate(end)
    }    
    fn initial_investment(&mut self){
        for (val1, _) in self.assets.iter_mut(){
            *val1 = 0.0;            
        }        
        self.assets.last_mut().unwrap().0 = self.initial_capital;
    }
}
