use std::env;
mod stock_file;

mod backtester;
mod rebalance_portfolio;
mod buy_hold_portfolio;

use stock_file::StockFile;
use backtester::Backtester;
use rebalance_portfolio::RebalancePortfolio;
use buy_hold_portfolio::BuyHoldPortfolio;

fn sorting_final_capital(capital_profit_vec : &[Option<f64>]) -> Vec<usize>{    
    let mut indices : Vec<usize> = capital_profit_vec.iter().enumerate().filter_map(|(i, val)| if val.is_some() {Some(i)} else {None}).collect();

    indices.sort_unstable_by(|&i, &j| {
        let a = capital_profit_vec[i].unwrap();
        let b = capital_profit_vec[j].unwrap();
        b.total_cmp(&a)
    });
    indices
}

fn maximum_profit(capital_profit_vec : &[Option<f64>]) -> usize{
    capital_profit_vec.iter().enumerate().fold(0, |acc, (id, val)|{
        match (capital_profit_vec[acc], *val){
            (Some(val_a), Some(val_b)) => if val_a > val_b { acc } else {id},
            (None, Some(_)) => id,
            (_, _) => acc,
        }
    })
}
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
            let max_indices = sorting_final_capital(&tmp_rolling);
            //let max_id = maximum_profit(&tmp_rolling);
            let max_id = max_indices[0];
            let cap = tmp_rolling[max_id].unwrap();

            let profits : Vec<f64> = max_indices.iter().map(|id| 
                        match tmp_rolling[*id] {
                            Some(val) => val / initial_capital,
                            None => 0.0,
                        }).collect();
            let mdds : Vec<f64> = max_indices.iter().map(|id| if *id > *duration { prices[id - *duration] / prev_maximum[id - *duration] } else { -1.0 }).collect();
            let dates : Vec<String> = max_indices.iter().map(|id| if *id > *duration { date_vec[id - *duration].clone() } else { "None".to_string() }).collect();
        
            file.write_sorted_vec(1, (cnt_strategy, 1), &format!("리밸런싱({}), 기간 {}", p.diff_ratio, *duration), &dates, &profits, &mdds);
            cnt_strategy += 4;
            /*file.merge_cells(1, (cnt_stategy, 1), (cnt_stategy + 3, 1));
            file.write_header(1, (cnt_stategy, 1), &format!("리밸런싱({}), 기간 {}", p.diff_ratio, *duration));
            file.write_header(1, (cnt_stategy, 2), "평균수익률");
            file.write_header(1, (cnt_stategy + 1, 2), "플러스수익확률");
            file.write_header(1, (cnt_stategy + 2, 2), "최대수익률");
            file.write_header(1, (cnt_stategy + 3, 2), "날짜");

            file.write_rate(0, (cnt_stategy, 3), avg / initial_capital);
            file.write_rate(0, (cnt_stategy + 1, 3), perc);
            file.write_rate(0, (cnt_stategy + 2, 3), cap / initial_capital);
            file.write_header(0, (cnt_stategy + 3, 3), &date_vec[max_id]);            
            cnt_stategy += 4;*/
            
            println!("리밸런싱({}), 기간 {} : 평균수익률 = {:.2}, 플러스수익확률 = {:.2}, 최대수익률 = {:.2}, 전고점대비 = {:.2}, 시작날짜 = {}", p.diff_ratio, duration, avg / initial_capital, perc, cap / initial_capital, prices[max_id - *duration] / prev_maximum[max_id - *duration], date_vec[max_id - *duration]);
        }
    }

    let mut buy_and_hold = BuyHoldPortfolio::new(initial_capital, fee_rate);
    for duration in vec_duration.iter(){
        let tmp_rolling = buy_and_hold.rolling_return(&prices, *duration);
        let max_indices = sorting_final_capital(&tmp_rolling);
        let (avg, _, _, perc) = analyze(&tmp_rolling, initial_capital);
        let max_id = max_indices[0];            
        //let max_id = maximum_profit(&tmp_rolling);
        let c = tmp_rolling[max_id].unwrap();

        let profits : Vec<f64> = max_indices.iter().map(|id| 
                        match tmp_rolling[*id] {
                            Some(val) => val / initial_capital,
                            None => 0.0,
                        }).collect();
        let mdds : Vec<f64> = max_indices.iter().map(|id| if *id > *duration { prices[max_id - *duration] / prev_maximum[max_id - *duration] } else { -1.0 }).collect();
        let dates : Vec<String> = max_indices.iter().map(|id| if *id > *duration { date_vec[max_id - *duration].clone() } else { "None".to_string() }).collect();
        
        file.write_sorted_vec(1, (cnt_strategy, 1), &format!("보유, 기간 {}", *duration), &dates, &profits, &mdds);
        cnt_strategy += 4;
               
        println!("보유, 기간 {} : 평균수익률 = {:.2}, 플러스수익확률 = {:.2}, 최대수익률 = {:.2}, 전고점대비 = {:.2}, 시작날짜 = {}", duration, avg / initial_capital, perc, c / initial_capital, prices[max_id - *duration] / prev_maximum[max_id - *duration], date_vec[max_id - *duration]);
    }
    file.write_xlsx();
    
}