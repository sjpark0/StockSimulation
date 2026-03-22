use crate::backtester::Backtester;
use crate::types::CapitalReturns;
use std::time::Instant;

pub struct RebalancePortfolio{
    initial_capital : f64,
    cash: f64,
    stock_qty: u32,
    pub threshold: f64,
    stock_ratio : f64,
    fee_rate : f64,
    price_history : Vec<f64>,
}

impl RebalancePortfolio{    
    pub fn new(initial_capital: f64, threshold: f64, stock_ratio: f64, fee_rate: f64, price_history : Vec<f64>) -> Self{
        Self { initial_capital : initial_capital, cash : initial_capital, stock_qty : 0, threshold : threshold, stock_ratio : stock_ratio, fee_rate : fee_rate, price_history : price_history}
    }
    fn rebalance(&mut self, current_price: f64, total_value: f64){
        let tmp_stock_value = total_value * self.stock_ratio;
        let tmp_qty = (tmp_stock_value / current_price).floor() as u32;
        let fee = ((self.stock_qty as f64) - (tmp_qty as f64)).abs() * self.fee_rate * 0.01;
        self.stock_qty = tmp_qty;
        self.cash = total_value - (self.stock_qty as f64) * current_price - fee;
        
    }    
        fn process_price(&mut self, current_price: f64){
        //let start_time = Instant::now();

        let stock_value = (self.stock_qty as f64) * current_price;
        let total_value = stock_value + self.cash;

        let stock_weight = stock_value / total_value;
        let cash_weight = self.cash / total_value;
        
        if(stock_weight - cash_weight).abs() >= self.threshold{
            self.rebalance(current_price, total_value);
        }

    }
    
    fn get_total_rate(&self, current_price: f64) -> (f64, f64) {
        let total_val = self.cash + (self.stock_qty as f64) * current_price;
        let profit = total_val / self.initial_capital;
        (total_val, profit)
    }

}

impl Backtester for RebalancePortfolio{
    fn rolling_return(&mut self, duration : usize) -> CapitalReturns{
        let length = self.price_history.len();
        CapitalReturns((0..length).map(|idx| if idx < duration { None } else { Some(self.process_backtester(idx - duration, idx).0) }).collect())            
    }
    fn process_backtester(&mut self, start : usize, end : usize) -> (f64, f64){
        self.initial_investment();        
        for i in start..=end{
            self.process_price(self.price_history[i]);
        }
        self.get_total_rate(self.price_history[end])
    }
        
    fn initial_investment(&mut self){
        self.cash = self.initial_capital;
        self.stock_qty = 0;
    }
}

