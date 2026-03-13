use std::env;
mod stock_file;
mod portfolio;

use portfolio::Portfolio;

fn main(){
    let args: Vec<String> = env::args().collect();
    let file_name = format!("{}.xlsx", args[1]);
    let prices = stock_file::load_excel_file(&file_name);
    
    let initial_capital = 100_000.0;
    let threshold = vec![0.1, 0.2, 0.3, 0.4, 0.5];
    
    let initial_price = prices[0];
    let initial_qty = (initial_capital / initial_price).round();
    let initial_cash = initial_capital - initial_price * initial_qty;

    let mut backtester: Vec<Portfolio> = vec![];
    for t in &threshold{
        backtester.push(Portfolio::new(initial_capital, initial_price, *t));
    }

    let mut val_vec = vec![0.0;threshold.len() + 2];
    let mut acc_win = vec![0;threshold.len() + 2];
    
    for current_price in prices.iter(){
        let buy_and_hold_value = initial_qty * current_price + initial_cash;
        
        val_vec[threshold.len()] = buy_and_hold_value;
        val_vec[threshold.len() + 1] = initial_capital;
        
        for (j, tester) in backtester.iter_mut().enumerate(){
            tester.process_price(*current_price);
            let current_value = tester.get_total_value(*current_price);
            val_vec[j] = current_value;
        }

        let (idx, max_val) = val_vec.iter().enumerate().fold((threshold.len() + 1, val_vec[threshold.len() + 1]), |(acc_id, acc_val), (id, &val)| if val > acc_val { (id, val)} else {(acc_id, acc_val)});
        acc_win[idx] += 1;
    }
    for idx in 0..threshold.len(){
        println!("리밸런싱({})자산 : {}", threshold[idx] * 100.0, acc_win[idx]);
    }
    println!("단순보유 : {}", acc_win[threshold.len()]);
    println!("현금 : {}", acc_win[threshold.len()+1]);
    
}