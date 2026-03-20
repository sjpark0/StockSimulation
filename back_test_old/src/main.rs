mod stock_file;
mod backtester;
mod rebalance_portfolio;
mod buy_hold_portfolio;
mod types;

use std::env;

use stock_file::StockFile;
use backtester::Backtester;
use rebalance_portfolio::RebalancePortfolio;
use buy_hold_portfolio::BuyHoldPortfolio;
use types::{CapitalReturns, PortfolidIndices};

fn analyze(capital_profit_vec : &[Option<f64>], initial_price : f64) -> (f64, f64, u32, f64){
    let valid_capital_profit_vec : Vec<f64> = capital_profit_vec.iter().filter(|&x| x.is_some()).map(|y| y.unwrap()).collect();
    let cnt = valid_capital_profit_vec.len() as f64;
    let avg_capital = valid_capital_profit_vec.iter().fold(0.0, |acc, c| acc + *c / cnt);
    let std_capital = valid_capital_profit_vec.iter().fold(0.0, |acc, c| acc + (avg_capital - *c) * (avg_capital - *c) / cnt);
    let num_win = valid_capital_profit_vec.iter().fold(0, |acc, c| if *c > initial_price {acc + 1} else {acc});    
    (avg_capital, std_capital.sqrt(), num_win, (num_win as f64) / cnt)    
}

fn main(){
    let args: Vec<String> = env::args().collect();
    let file_name = format!("{}.xlsx", args[1]);
    let mut file = StockFile::new(&file_name);
    let (date_vec, prices, prev_maximum) = file.load(0);
    let fee_rate = 0.25;
    let initial_capital = 100_000.0;
    let threshold = vec![0.1, 0.2, 0.3];
    let vec_duration: Vec<usize> = vec![250, 500];

    let mut portfolio_vec :Vec<RebalancePortfolio> = threshold.iter().map(|x| RebalancePortfolio::new(initial_capital, *x, 0.5, fee_rate)).collect();
    let mut cnt_strategy = 1;
    for p in portfolio_vec.iter_mut(){
        for duration in vec_duration.iter(){
            let tmp_rolling = p.rolling_return(&prices, *duration);
            let (avg, _, _, perc) = analyze(&tmp_rolling, initial_capital);
            //let max_indices = p.remove_redundant(&p.sorting_final_capital(&tmp_rolling));
            let max_indices = tmp_rolling.sorting_final_capital().remove_redundant_from_maximum();
            let min_indices = tmp_rolling.sorting_final_capital().remove_redundant_from_minimum();
            let max_id = max_indices[0];
            let min_id = min_indices[0];
            let cap = tmp_rolling[max_id].unwrap();
            let cap_min = tmp_rolling[min_id].unwrap();
            
            let profits : Vec<f64> = max_indices.iter().map(|id| tmp_rolling[*id].unwrap() / initial_capital).collect();                        
            let mdds : Vec<f64> = max_indices.iter().map(|id| prices[id - *duration] / prev_maximum[id - *duration]).collect();
            let dates : Vec<String> = max_indices.iter().map(|id| date_vec[id - *duration].clone()).collect();
            
        
            file.write_sorted_vec(1, (cnt_strategy, 1), &format!("리밸런싱({}), 기간 {}", p.diff_ratio, *duration), &dates, &profits, &mdds);
            cnt_strategy += 4;
            
            println!("리밸런싱({}), 기간 {} : 평균수익률 = {:.2}, 플러스수익확률 = {:.2}, 최대수익률 = {:.2}, 전고점대비 = {:.2}, 시작날짜 = {}, 최소수익률 = {:.2}, 전고점대비 = {:.2}, 시작날짜 = {}", p.diff_ratio, duration, avg / initial_capital, perc, cap / initial_capital, prices[max_id - *duration] / prev_maximum[max_id - *duration], date_vec[max_id - *duration], cap_min / initial_capital, prices[min_id - *duration] / prev_maximum[min_id - *duration], date_vec[min_id - *duration]);
        }
    }

    let mut buy_and_hold = BuyHoldPortfolio::new(initial_capital, fee_rate);
    for duration in vec_duration.iter(){
        let tmp_rolling = buy_and_hold.rolling_return(&prices, *duration);
        let max_indices = tmp_rolling.sorting_final_capital().remove_redundant_from_maximum();
        let min_indices = tmp_rolling.sorting_final_capital().remove_redundant_from_minimum();
                
        let (avg, _, _, perc) = analyze(&tmp_rolling, initial_capital);
        let max_id = max_indices[0];            
        let min_id = min_indices[0];
                
        let c = tmp_rolling[max_id].unwrap();
        let cap_min = tmp_rolling[min_id].unwrap();
        let profits : Vec<f64> = max_indices.iter().map(|id| tmp_rolling[*id].unwrap() / initial_capital).collect();                        
        let mdds : Vec<f64> = max_indices.iter().map(|id| prices[id - *duration] / prev_maximum[id - *duration]).collect();
        let dates : Vec<String> = max_indices.iter().map(|id| date_vec[id - *duration].clone()).collect();

        file.write_sorted_vec(1, (cnt_strategy, 1), &format!("보유, 기간 {}", *duration), &dates, &profits, &mdds);
        cnt_strategy += 4;
               
        println!("보유, 기간 {} : 평균수익률 = {:.2}, 플러스수익확률 = {:.2}, 최대수익률 = {:.2}, 전고점대비 = {:.2}, 시작날짜 = {}, 최소수익률 = {:.2}, 전고점대비 = {:.2}, 시작날짜 = {}", duration, avg / initial_capital, perc, c / initial_capital, prices[max_id - *duration] / prev_maximum[max_id - *duration], date_vec[max_id - *duration], cap_min / initial_capital, prices[min_id - *duration] / prev_maximum[min_id - *duration], date_vec[min_id - *duration]);
    }
    file.write_xlsx();
    
}