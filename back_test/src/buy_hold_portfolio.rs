use crate::backtester::Backtester;

pub struct BuyHoldPortfolio{
    initial_capital : f64,
    fee_rate : f64,
    cash: f64,
    stock_qty: u32,    
}

impl BuyHoldPortfolio{    
    pub fn new(initial_capital: f64, fee_rate: f64) -> Self{
        Self { initial_capital : initial_capital, cash : initial_capital, stock_qty : 0, fee_rate : fee_rate}
    }
    
}

impl Backtester for BuyHoldPortfolio{
    fn process_backtester(&mut self, price_history : &[f64], start : usize, end : usize) -> (f64, f64){
        self.initial_investment();
        self.process_price(price_history[start]);
        self.get_total_rate(price_history[end])
    }
    fn rolling_return(&mut self, price_history : &[f64], duration : usize) -> Vec<Option<f64>>{
        let mut res_vec: Vec<Option<f64>> = Vec::new();        
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
    
    fn initial_investment(&mut self){
        self.cash = self.initial_capital;
        self.stock_qty = 0;
    }
        
    fn process_price(&mut self, current_price: f64){
        self.stock_qty = (self.initial_capital / current_price).floor() as u32;
        self.cash = self.initial_capital - (self.stock_qty as f64) * current_price * (1.0 + self.fee_rate * 0.01);
    }
    
    fn get_total_rate(&self, current_price: f64) -> (f64, f64) {
        let total_val = self.cash + (self.stock_qty as f64) * current_price;
        let profit = total_val / self.initial_capital;
        (total_val, profit)
    }
    
}