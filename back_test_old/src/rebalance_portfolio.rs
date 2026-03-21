use crate::backtester::Backtester;
use std::time::Instant;

pub struct RebalancePortfolio{
    initial_capital : f64,
    cash: f64,
    stock_qty: u32,
    pub diff_ratio: f64,
    stock_ratio : f64,
    fee_rate : f64,
}

impl RebalancePortfolio{    
    pub fn new(initial_capital: f64, diff_ratio: f64, stock_ratio: f64, fee_rate: f64) -> Self{
        Self { initial_capital : initial_capital, cash : initial_capital, stock_qty : 0, diff_ratio : diff_ratio, stock_ratio : stock_ratio, fee_rate : fee_rate}
    }
    fn rebalance(&mut self, current_price: f64, total_value: f64){
        let start_time = Instant::now();

        let tmp_stock_value = total_value * self.stock_ratio;
        let tmp_qty = (tmp_stock_value / current_price).floor() as u32;
        let fee = ((self.stock_qty as f64) - (tmp_qty as f64)).abs() * self.fee_rate * 0.01;
        self.stock_qty = tmp_qty;
        self.cash = total_value - (self.stock_qty as f64) * current_price - fee;
        
    }    
}

impl Backtester for RebalancePortfolio{
    fn process_backtester(&mut self, price_history : &[f64], start : usize, end : usize) -> (f64, f64){
        self.initial_investment();
        
        for i in start..=end{
            self.process_price(price_history[i]);
        }

        self.get_total_rate(price_history[end])
    }
        
    fn initial_investment(&mut self){
        self.cash = self.initial_capital;
        self.stock_qty = 0;
    }
    fn process_price(&mut self, current_price: f64){
        //let start_time = Instant::now();

        let stock_value = (self.stock_qty as f64) * current_price;
        let total_value = stock_value + self.cash;

        let stock_weight = stock_value / total_value;
        let cash_weight = self.cash / total_value;

        //let duration = start_time.elapsed();
        //println!("process price : 실행 시간: {:.6}초 ({}ms, {}ns)", duration.as_secs_f64(), duration.as_millis(), duration.as_nanos());

        if(stock_weight - cash_weight).abs() >= self.diff_ratio{
            self.rebalance(current_price, total_value);
        }

    }
    
    fn get_total_rate(&self, current_price: f64) -> (f64, f64) {
        let total_val = self.cash + (self.stock_qty as f64) * current_price;
        let profit = total_val / self.initial_capital;
        (total_val, profit)
    }
}

