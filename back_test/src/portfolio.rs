use crate::portfolio;

pub struct Portfolio{
    cash: f64,
    stock_qty: u32,
    threshold: f64,
    ratio : f64,
    fee_rate : f64,
}

impl Portfolio{
    pub fn new(initial_capital: f64, initial_price: f64, threshold: f64, ratio: f64, fee_rate: f64) -> Self{
        let tmp_stock_value = initial_capital * ratio;
        let qty = (tmp_stock_value / initial_price).round() as u32;
        let cash = initial_capital - (qty as f64) * initial_price * (1.0 + fee_rate * 0.01);

        Self { cash: cash, stock_qty: qty, threshold: threshold, ratio : ratio, fee_rate: fee_rate}
    }
    
    pub fn process_price(&mut self, current_price: f64){
        let stock_value = (self.stock_qty as f64) * current_price;
        let total_value = stock_value + self.cash;

        let stock_weight = stock_value / total_value;
        let cash_weight = self.cash / total_value;

        if(stock_weight - cash_weight).abs() >= self.threshold{
            self.rebalance(current_price, total_value);
        }
    }
    pub fn rebalance(&mut self, current_price: f64, total_value: f64){
        let tmp_stock_value = total_value * self.ratio;
        let tmp_qty = (tmp_stock_value / current_price).round() as u32;
        let fee = ((self.stock_qty as f64) - (tmp_qty as f64)).abs() * self.fee_rate * 0.01;
        self.stock_qty = tmp_qty;
        self.cash = total_value - (self.stock_qty as f64) * current_price - fee;
    }
    pub fn get_total_value(&self, current_price: f64) -> f64{
        self.cash + (self.stock_qty as f64) * current_price
    }
    
}

pub trait Rate {
    fn get_profit_rate(&self, initial_value : f64) -> f64;
}
impl Rate for f64{
    fn get_profit_rate(&self, initial_value : f64) -> f64{
        (*self / initial_value) - 1.0
    }
}

pub trait Backtester{
    fn process_backtester(&self, price_history : &[f64], initial_capital : f64, start : usize, end : usize, stock_ratio : f64, fee_rate : f64) -> (f64, f64);
    fn rolling_return(&self, price_history : &[f64], initial_capital : f64, stock_ratio : f64, duration : usize, fee_rate : f64) -> Vec<Option<(f64, f64, f64)>>;
}
impl Backtester for f64{
    fn process_backtester(&self, price_history : &[f64], initial_capital : f64, start : usize, end : usize, stock_ratio : f64, fee_rate : f64) -> (f64, f64){
        let mut portfolio = Portfolio::new(initial_capital, price_history[start], *self, stock_ratio, fee_rate);
        for i in start..=end{
            portfolio.process_price(price_history[i]);
        }
        let final_capital = portfolio.get_total_value(price_history[end]);   
        let profit = final_capital.get_profit_rate(initial_capital);
        (final_capital, profit)
    }
    fn rolling_return(&self, price_history : &[f64], initial_capital : f64, stock_ratio : f64, duration : usize, fee_rate : f64) -> Vec<Option<(f64, f64, f64)>>{
        let mut res_vec: Vec<Option<(f64, f64, f64)>> = Vec::new();
        
        let mut prev_maximum: f64 = 0.0;
        for i in 0..price_history.len(){
            if i < duration{
                res_vec.push(None);
            }
            else{
                prev_maximum = prev_maximum.max(price_history[i]);
                let (cap, profit) = self.process_backtester(price_history, initial_capital, i - duration, i, stock_ratio, fee_rate);
                res_vec.push(Some((cap, profit, price_history[i] / prev_maximum)));
            }
        }
        res_vec
    }
}