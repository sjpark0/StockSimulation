pub trait Backtester{    
    fn process_backtester(&mut self, price_history : &[f64], start : usize, end : usize) -> (f64, f64);
    fn rolling_return(&mut self, price_history : &[f64], duration : usize) -> Vec<Option<f64>>;
    fn initial_investment(&mut self);
    fn process_price(&mut self, current_price : f64);    
    fn get_total_rate(&self, current_price: f64) -> (f64, f64);
}