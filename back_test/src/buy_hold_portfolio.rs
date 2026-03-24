use crate::backtester::Backtester;
use crate::types::{CapitalReturns, StockPrices};

pub struct BuyHoldPortfolio{
    initial_capital : f64,
    fee_rate : f64,
    cash: f64,
    stock_qty: u32,    
    price_history : StockPrices,
}

impl BuyHoldPortfolio{    
    pub fn new(initial_capital: f64, fee_rate: f64, price_history : StockPrices) -> Self{
        Self { initial_capital : initial_capital, cash : initial_capital, stock_qty : 0, fee_rate : fee_rate, price_history : price_history}
    }
    fn process_price(&mut self, current_price: f64){
        self.stock_qty = (self.initial_capital / current_price).floor() as u32;
        self.cash = self.initial_capital - (self.stock_qty as f64) * current_price * (1.0 + self.fee_rate * 0.01);
    }
}

impl Backtester for BuyHoldPortfolio{
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
            let total_val = (self.stock_qty as f64) * self.price_history[i] + self.cash;
            local_maximum = local_maximum.max(total_val);
            mdd = mdd.max(1.0 - total_val / local_maximum);         
        }
        let total_val = (self.stock_qty as f64) * self.price_history[end] + self.cash;
        (total_val, mdd)        

    }
    
    fn initial_investment(&mut self){
        self.cash = self.initial_capital;
        self.stock_qty = 0;
    }
        
    
}