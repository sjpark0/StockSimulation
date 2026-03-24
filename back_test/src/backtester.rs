use crate::types::CapitalReturns;

pub trait Backtester{    
    fn process_backtester(&mut self, start : usize, end : usize) -> (f64, f64);
    fn rolling_return(&mut self, duration : usize) -> CapitalReturns;    
    fn initial_investment(&mut self);
}

