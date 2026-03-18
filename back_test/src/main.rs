mod stock_file;
mod backtester;
mod rebalance_portfolio;
mod buy_hold_portfolio;

use std::env;
use std::collections::VecDeque;
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

fn remove_redundant(sorted_idx : &[usize]) -> Vec<usize>{
    let mut res_idx = VecDeque::with_capacity(sorted_idx.len());
    res_idx.extend(sorted_idx.iter());
    //let mut res_idx = vec![0;sorted_idx.len()];
    let mut res_idx1 = Vec::new();
    //res_idx.clone_from_slice(sorted_idx);
    let mut loop_var = 0;
    /*while loop_var < res_idx.len(){
        let num = res_idx[loop_var];
        res_idx.retain(|&x| x > num + 21 || x < num - 21);    
        res_idx1.push(num);
        loop_var += 1;
    }*/
    while let Some(num) = res_idx.pop_front(){
        res_idx.retain(|&x| x > num + 21 || x < num - 21);    
        res_idx1.push(num);        
    }
    
    res_idx1
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
            //let max_indices = sorting_final_capital(&tmp_rolling);
            let max_indices = remove_redundant(&sorting_final_capital(&tmp_rolling));
            //let max_id = maximum_profit(&tmp_rolling);
            let max_id = max_indices[0];
            let min_id = max_indices[max_indices.len() - 1];
            let cap = tmp_rolling[max_id].unwrap();
            let cap_min = tmp_rolling[min_id].unwrap();
            
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
            
            println!("리밸런싱({}), 기간 {} : 평균수익률 = {:.2}, 플러스수익확률 = {:.2}, 최대수익률 = {:.2}, 전고점대비 = {:.2}, 시작날짜 = {}, 최소수익률 = {:.2}, 전고점대비 = {:.2}, 시작날짜 = {}", p.diff_ratio, duration, avg / initial_capital, perc, cap / initial_capital, prices[max_id - *duration] / prev_maximum[max_id - *duration], date_vec[max_id - *duration], cap_min / initial_capital, prices[min_id - *duration] / prev_maximum[min_id - *duration], date_vec[min_id - *duration]);
        }
    }

    let mut buy_and_hold = BuyHoldPortfolio::new(initial_capital, fee_rate);
    for duration in vec_duration.iter(){
        let tmp_rolling = buy_and_hold.rolling_return(&prices, *duration);
        let max_indices = remove_redundant(&sorting_final_capital(&tmp_rolling));
            
        //let max_indices = sorting_final_capital(&tmp_rolling);
        let (avg, _, _, perc) = analyze(&tmp_rolling, initial_capital);
        let max_id = max_indices[0];            
        let min_id = max_indices[max_indices.len() - 1];
            
        //let max_id = maximum_profit(&tmp_rolling);
        let c = tmp_rolling[max_id].unwrap();
        let cap_min = tmp_rolling[min_id].unwrap();
        let profits : Vec<f64> = max_indices.iter().map(|id| 
                        match tmp_rolling[*id] {
                            Some(val) => val / initial_capital,
                            None => 0.0,
                        }).collect();
        let mdds : Vec<f64> = max_indices.iter().map(|id| if *id > *duration { prices[id - *duration] / prev_maximum[id - *duration] } else { -1.0 }).collect();
        let dates : Vec<String> = max_indices.iter().map(|id| if *id > *duration { date_vec[id - *duration].clone() } else { "None".to_string() }).collect();
        file.write_sorted_vec(1, (cnt_strategy, 1), &format!("보유, 기간 {}", *duration), &dates, &profits, &mdds);
        cnt_strategy += 4;
               
        println!("보유, 기간 {} : 평균수익률 = {:.2}, 플러스수익확률 = {:.2}, 최대수익률 = {:.2}, 전고점대비 = {:.2}, 시작날짜 = {}, 최소수익률 = {:.2}, 전고점대비 = {:.2}, 시작날짜 = {}", duration, avg / initial_capital, perc, c / initial_capital, prices[max_id - *duration] / prev_maximum[max_id - *duration], date_vec[max_id - *duration], cap_min / initial_capital, prices[min_id - *duration] / prev_maximum[min_id - *duration], date_vec[min_id - *duration]);
    }
    file.write_xlsx();
    
}