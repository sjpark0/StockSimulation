use crate::backtester::Backtester;
use crate::types::{Assets, CapitalReturns};

pub struct RebalancePortfolioVec{
    initial_capital : f64,
    assets : Assets,
    fee_rate : f64,
    pub threshold: f64,    
    price_history : Vec<f64>,
}

impl RebalancePortfolioVec{    
    pub fn new(initial_capital: f64, tickers_fraction : &[f64], fee_rate : f64, threshold: f64, price_histroy : Vec<f64>) -> Self{
        let mut assets = Assets(Vec::new());
        let mut fractions = 1.0;
        for f in tickers_fraction.iter(){
            assets.push((0.0, *f));
            fractions -= *f;
        }
        assets.push((0.0, fractions));
        Self { initial_capital : initial_capital, assets : assets, fee_rate : fee_rate, threshold : threshold, price_history : price_histroy}

    }
    fn rebalance(&mut self, current_price: f64, total_value: f64){
        let tmp_stock_value = total_value * self.assets[0].1;
        let pre_qty = self.assets[0].0;
        self.assets[0].0 = (tmp_stock_value / current_price).floor();
        let fee = (self.assets[0].0 - pre_qty).abs() * self.fee_rate * 0.01;
        self.assets[1].0 = total_value - self.assets[0].0 * current_price - fee;                
    }    
    fn process_price(&mut self, current_price: f64){
        let qty = self.assets[0].0;
        let cash = self.assets[1].0;
        let stock_value = qty * current_price;
        let total_val = cash + stock_value;
        
        let stock_weight = stock_value / total_val;
        let cash_weight = cash / total_val;
        let total_ratio = (stock_weight - self.assets[0].1).abs() + (cash_weight - self.assets[1].1).abs();
        
        if total_ratio >= self.threshold{
        //if(stock_weight - cash_weight).abs() >= self.threshold{
            self.rebalance(current_price, total_val);
        }

    }
    
    fn get_total_rate(&self, current_price : f64) -> (f64, f64) {
        let total_val = self.assets[1].0 + self.assets[0].0 * current_price;
        (total_val, total_val / self.initial_capital)
    }

}

impl Backtester for RebalancePortfolioVec{
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
        for (val1, _) in self.assets.iter_mut(){
            *val1 = 0.0;            
        }        
        if let Some((val1, _)) = self.assets.last_mut(){
            *val1 = self.initial_capital;
        }    
    }
}
