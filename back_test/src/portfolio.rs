pub struct Portfolio{
    cash: f64,
    stock_qty: u32,
    threshold: f64,
}

impl Portfolio{
    pub fn new(initial_capital: f64, initial_price: f64, threshold: f64) -> Self{
        let half_capital = initial_capital / 2.0;
        let qty = (half_capital / initial_price).round() as u32;
        let cash = initial_capital - (qty as f64) * initial_price;
        Self { cash: cash, stock_qty: qty, threshold: threshold}
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
        let half_capital = total_value / 2.0;
        self.stock_qty = (half_capital / current_price).round() as u32;
        self.cash = total_value - (self.stock_qty as f64) * current_price;
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