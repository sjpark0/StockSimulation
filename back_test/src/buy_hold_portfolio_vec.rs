use crate::backtester::Backtester;
use crate::types::{Assets, CapitalReturns, StockPrices};

pub struct BuyHoldPortfolioVec{
    initial_capital : f64,
    assets : Assets,
    fee_rate : f64,
    price_history : StockPrices,
}

impl BuyHoldPortfolioVec{    
    pub fn new(initial_capital: f64, fee_rate: f64, price_history : StockPrices) -> Self{
        let mut assets = Assets(Vec::new());
        assets.push((0.0, 1.0));
        assets.push((0.0, 0.0));
        Self { initial_capital : initial_capital, assets : assets, fee_rate : fee_rate, price_history : price_history}
    }    
    fn process_price(&mut self, current_price : f64){
        let mut total_stock = 0.0;

        self.assets[0].0 = (self.initial_capital / current_price).floor();
        self.assets[1].0 = self.initial_capital - self.assets[0].0 * current_price * (1.0 + self.fee_rate * 0.01);        
    }
    
    fn get_total_rate(&self, current_price : f64) -> (f64, f64) {
        let total_val = self.assets[1].0 + self.assets[0].0 * current_price;
        (total_val, total_val / self.initial_capital)
    }
}

impl Backtester for BuyHoldPortfolioVec{
    fn rolling_return(&mut self, duration : usize) -> CapitalReturns{
        let length = self.price_history.len();
        CapitalReturns((0..length).map(|idx| if idx < duration { None } else { Some(self.process_backtester(idx - duration, idx)) }).collect())            
    }
    
    fn process_backtester(&mut self, start : usize, end : usize) -> (f64, f64){
        let mut local_maximum: f64 = 0.0;
        let mut mdd: f64 = 0.0;

        self.initial_investment();
        self.process_price(self.price_history[start]);
        for i in start..end{
            let total_val = self.assets[1].0 + self.assets[0].0 * self.price_history[i];
            local_maximum = local_maximum.max(total_val);
            mdd = mdd.max(1.0 - total_val / local_maximum);         
        }
        let total_val = self.assets[1].0 + self.assets[0].0 * self.price_history[end];
        (total_val, mdd)
    }
    
    fn initial_investment(&mut self){
        for (val1, _) in self.assets.iter_mut(){
            *val1 = 0.0;            
        }        
        if let Some((val1, _)) = self.assets.last_mut(){
            *val1 = self.initial_capital;
        }
    }
}