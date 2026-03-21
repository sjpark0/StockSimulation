use crate::types::CapitalReturns;
use std::time::Instant;


pub trait Backtester{    
    fn process_backtester(&mut self, price_history : &[f64], start : usize, end : usize) -> (f64, f64);
    fn rolling_return(&mut self, price_history : &[f64], duration : usize) -> CapitalReturns{        
        let mut res_vec: CapitalReturns = CapitalReturns(Vec::new());        
        for i in 0..price_history.len(){
            if i < duration{
                res_vec.push(None);
            }
            else{
                let (cap, _) = self.process_backtester(price_history, i - duration, i);
                res_vec.push(Some(cap));
            }
        }
        res_vec
    }
    
    
    fn initial_investment(&mut self);
    fn process_price(&mut self, current_price : f64);    
    fn get_total_rate(&self, current_price: f64) -> (f64, f64);
}

