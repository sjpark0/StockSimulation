mod types;
mod backtester;
mod stock_file;

mod rebalance_portfolio;
mod buy_hold_portfolio;

mod rebalance_portfolio_vec;
mod buy_hold_portfolio_vec;

mod rebalance_portfolio_hashmap;
mod buy_hold_portfolio_hashmap;

mod rebalance_portfolio_vec2;
mod buy_hold_portfolio_vec2;

mod rebalance_portfolio_vec_bollinger;

use std::env;
use std::collections::HashMap;

use stock_file::StockFile;
use backtester::Backtester;

use rebalance_portfolio::RebalancePortfolio;
use buy_hold_portfolio::BuyHoldPortfolio;

use rebalance_portfolio_vec::RebalancePortfolioVec;
use buy_hold_portfolio_vec::BuyHoldPortfolioVec;

use rebalance_portfolio_hashmap::RebalancePortfolioHashmap;
use buy_hold_portfolio_hashmap::BuyHoldPortfolioHashmap;

use rebalance_portfolio_vec2::RebalancePortfolioVec2;
use buy_hold_portfolio_vec2::BuyHoldPortfolioVec2;
use rebalance_portfolio_vec_bollinger::RebalancePortfolioVecBollinger;

use types::StockPrices;

//use types::{Assets, CapitalReturns, PortfolidIndices};
use std::time::Instant;

use crate::types::BollingerBand;

fn analyze(capital_profit_vec : &[Option<f64>], initial_price : f64) -> (f64, f64, u32, f64){
    let valid_capital_profit_vec : Vec<f64> = capital_profit_vec.iter().filter(|&x| x.is_some()).map(|y| y.unwrap()).collect();
    let cnt = valid_capital_profit_vec.len() as f64;
    let avg_capital = valid_capital_profit_vec.iter().fold(0.0, |acc, c| acc + *c / cnt);
    let std_capital = valid_capital_profit_vec.iter().fold(0.0, |acc, c| acc + (avg_capital - *c) * (avg_capital - *c) / cnt);
    let num_win = valid_capital_profit_vec.iter().fold(0, |acc, c| if *c > initial_price {acc + 1} else {acc});    
    (avg_capital, std_capital.sqrt(), num_win, (num_win as f64) / cnt)    
}
fn main_hashmap(){
    let start = Instant::now();

    let args: Vec<String> = env::args().collect();
    let mut files = Vec::new();
    let mut price_histories: HashMap<String, StockPrices> = HashMap::new();

    for i in 1..args.len(){
        files.push(StockFile::new(&format!("{}.xlsx", args[i])));        
    }
    let (date_vec, prices, prev_maximum) = files[0].load(0);
    price_histories.insert(args[1].clone(), prices);

    for i in 2..args.len(){
        let (_, price_vec, _) = files[i-1].load(0);
        price_histories.insert(args[i].clone(), price_vec);
    }
    let fee_rate = 0.25;
    let initial_capital = 100_000.0;
    let threshold = vec![0.1, 0.2, 0.3];
    let vec_duration: Vec<usize> = vec![250, 500, 1000, 2000, 2500];

    let mut ticker_fraction = Vec::new();
    for i in 1..args.len(){
        ticker_fraction.push((args[i].to_string(), 1.0 / (args.len() as f64)));
    }

    let mut portfolio_vec :Vec<RebalancePortfolioHashmap> = threshold.iter().map(|x| RebalancePortfolioHashmap::new(initial_capital, &ticker_fraction, fee_rate, *x, price_histories.clone())).collect();

    let mut cnt_strategy = 1;
    for p in portfolio_vec.iter_mut(){
        for duration in vec_duration.iter(){
            let tmp_rolling = p.rolling_return(*duration);
            let (avg, _, _, perc) = analyze(&tmp_rolling, initial_capital);
            //let max_indices = p.remove_redundant(&p.sorting_final_capital(&tmp_rolling));
            let max_indices = tmp_rolling.sorting_final_capital().remove_redundant_from_maximum();
            let min_indices = tmp_rolling.sorting_final_capital().remove_redundant_from_minimum();
            let max_id = max_indices[0];
            let min_id = min_indices[0];
            let cap = tmp_rolling[max_id].unwrap();
            let cap_min = tmp_rolling[min_id].unwrap();
            
            //let profits : Vec<f64> = max_indices.iter().map(|id| tmp_rolling[*id].unwrap() / initial_capital).collect();                        
            //let mdds : Vec<f64> = max_indices.iter().map(|id| prices[id - *duration] / prev_maximum[id - *duration]).collect();
            //let dates : Vec<String> = max_indices.iter().map(|id| date_vec[id - *duration].clone()).collect();
            
        
            //file.write_sorted_vec(1, (cnt_strategy, 1), &format!("리밸런싱({}), 기간 {}", p.diff_ratio, *duration), &dates, &profits, &mdds);
            //cnt_strategy += 4;
            
            println!("리밸런싱({}), 기간 {} : 평균수익률 = {:.2}, 플러스수익확률 = {:.2}, 최대수익률 = {:.2}, 시작날짜 = {}, 최소수익률 = {:.2}, 시작날짜 = {}", p.threshold, duration, avg / initial_capital, perc, cap / initial_capital, date_vec[max_id - *duration], cap_min / initial_capital, date_vec[min_id - *duration]);
        }
    }

    let mut ticker_fraction = Vec::new();
    for i in 1..args.len(){
        ticker_fraction.push((args[i].to_string(), 1.0 / (args.len() as f64 - 1.0)));
    }

    let mut buy_and_hold = BuyHoldPortfolioHashmap::new(initial_capital, fee_rate, &ticker_fraction, price_histories.clone());
    for duration in vec_duration.iter(){
        let tmp_rolling = buy_and_hold.rolling_return(*duration);
        let max_indices = tmp_rolling.sorting_final_capital().remove_redundant_from_maximum();
        let min_indices = tmp_rolling.sorting_final_capital().remove_redundant_from_minimum();
                
        let (avg, _, _, perc) = analyze(&tmp_rolling, initial_capital);
        let max_id = max_indices[0];            
        let min_id = min_indices[0];
                
        let c = tmp_rolling[max_id].unwrap();
        let cap_min = tmp_rolling[min_id].unwrap();
        //let profits : Vec<f64> = max_indices.iter().map(|id| tmp_rolling[*id].unwrap() / initial_capital).collect();                        
        //let mdds : Vec<f64> = max_indices.iter().map(|id| prices[id - *duration] / prev_maximum[id - *duration]).collect();
        //let dates : Vec<String> = max_indices.iter().map(|id| date_vec[id - *duration].clone()).collect();

        //file.write_sorted_vec(1, (cnt_strategy, 1), &format!("보유, 기간 {}", *duration), &dates, &profits, &mdds);
        //cnt_strategy += 4;
               
        println!("보유, 기간 {} : 평균수익률 = {:.2}, 플러스수익확률 = {:.2}, 최대수익률 = {:.2}, 시작날짜 = {}, 최소수익률 = {:.2}, 시작날짜 = {}", duration, avg / initial_capital, perc, c / initial_capital, date_vec[max_id - *duration], cap_min / initial_capital, date_vec[min_id - *duration]);
    }
    //file.write_xlsx();
    let duration = start.elapsed();
    println!("실행 시간: {:.6}초 ({}ms)", duration.as_secs_f64(), duration.as_millis());
}
fn main_vec2(){
    let start = Instant::now();

    let args: Vec<String> = env::args().collect();
    let mut files = Vec::new();
    let mut price_histories: Vec<StockPrices> = Vec::new();

    for i in 1..args.len(){
        files.push(StockFile::new(&format!("{}.xlsx", args[i])));        
    }
    let (date_vec, prices, prev_maximum) = files[0].load(0);
    price_histories.push(prices);

    for i in 2..args.len(){
        let (_, price_vec, _) = files[i-1].load(0);
        price_histories.push(price_vec);
    }
    let fee_rate = 0.25;
    let initial_capital = 100_000.0;
    let threshold = vec![0.1, 0.2, 0.3];
    let vec_duration: Vec<usize> = vec![250, 500, 1000, 2000, 2500];

    let mut ticker_fraction = Vec::new();
    for i in 1..args.len(){
        ticker_fraction.push(1.0 / (args.len() as f64));
    }

    let mut portfolio_vec :Vec<RebalancePortfolioVec2> = threshold.iter().map(|x| RebalancePortfolioVec2::new(initial_capital, &ticker_fraction, fee_rate, *x, price_histories.clone())).collect();

    let mut cnt_strategy = 1;
    for p in portfolio_vec.iter_mut(){
        for duration in vec_duration.iter(){
            let tmp_rolling = p.rolling_return(*duration);
            let (avg, _, _, perc) = analyze(&tmp_rolling, initial_capital);
            //let max_indices = p.remove_redundant(&p.sorting_final_capital(&tmp_rolling));
            let max_indices = tmp_rolling.sorting_final_capital().remove_redundant_from_maximum();
            let min_indices = tmp_rolling.sorting_final_capital().remove_redundant_from_minimum();
            let max_id = max_indices[0];
            let min_id = min_indices[0];
            let cap = tmp_rolling[max_id].unwrap();
            let cap_min = tmp_rolling[min_id].unwrap();
            
            //let profits : Vec<f64> = max_indices.iter().map(|id| tmp_rolling[*id].unwrap() / initial_capital).collect();                        
            //let mdds : Vec<f64> = max_indices.iter().map(|id| prices[id - *duration] / prev_maximum[id - *duration]).collect();
            //let dates : Vec<String> = max_indices.iter().map(|id| date_vec[id - *duration].clone()).collect();
            
        
            //file.write_sorted_vec(1, (cnt_strategy, 1), &format!("리밸런싱({}), 기간 {}", p.diff_ratio, *duration), &dates, &profits, &mdds);
            //cnt_strategy += 4;
            
            println!("리밸런싱({}), 기간 {} : 평균수익률 = {:.2}, 플러스수익확률 = {:.2}, 최대수익률 = {:.2}, 시작날짜 = {}, 최소수익률 = {:.2}, 시작날짜 = {}", p.threshold, duration, avg / initial_capital, perc, cap / initial_capital, date_vec[max_id - *duration], cap_min / initial_capital, date_vec[min_id - *duration]);
        }
    }

    let mut ticker_fraction = Vec::new();
    for i in 1..args.len(){
        ticker_fraction.push(1.0 / (args.len() as f64 - 1.0));
    }

    let mut buy_and_hold = BuyHoldPortfolioVec2::new(initial_capital, fee_rate, &ticker_fraction, price_histories.clone());
    for duration in vec_duration.iter(){
        let tmp_rolling = buy_and_hold.rolling_return(*duration);
        let max_indices = tmp_rolling.sorting_final_capital().remove_redundant_from_maximum();
        let min_indices = tmp_rolling.sorting_final_capital().remove_redundant_from_minimum();
                
        let (avg, _, _, perc) = analyze(&tmp_rolling, initial_capital);
        let max_id = max_indices[0];            
        let min_id = min_indices[0];
                
        let c = tmp_rolling[max_id].unwrap();
        let cap_min = tmp_rolling[min_id].unwrap();
        //let profits : Vec<f64> = max_indices.iter().map(|id| tmp_rolling[*id].unwrap() / initial_capital).collect();                        
        //let mdds : Vec<f64> = max_indices.iter().map(|id| prices[id - *duration] / prev_maximum[id - *duration]).collect();
        //let dates : Vec<String> = max_indices.iter().map(|id| date_vec[id - *duration].clone()).collect();

        //file.write_sorted_vec(1, (cnt_strategy, 1), &format!("보유, 기간 {}", *duration), &dates, &profits, &mdds);
        //cnt_strategy += 4;
               
        println!("보유, 기간 {} : 평균수익률 = {:.2}, 플러스수익확률 = {:.2}, 최대수익률 = {:.2}, 시작날짜 = {}, 최소수익률 = {:.2}, 시작날짜 = {}", duration, avg / initial_capital, perc, c / initial_capital, date_vec[max_id - *duration], cap_min / initial_capital, date_vec[min_id - *duration]);
    }
    //file.write_xlsx();
    let duration = start.elapsed();
    println!("실행 시간: {:.6}초 ({}ms)", duration.as_secs_f64(), duration.as_millis());
}

fn main_bollinger(){
    let start = Instant::now();

    let args: Vec<String> = env::args().collect();
    let mut files = Vec::new();
    let mut price_histories: Vec<StockPrices> = Vec::new();
    let mut bollinger_bands : Vec<BollingerBand> = Vec::new();
    let num_avg = 21;
    let sigma = 2.0;

    for i in 1..args.len(){
        files.push(StockFile::new(&format!("{}.xlsx", args[i])));        
    }
    let (date_vec, prices, prev_maximum) = files[0].load(0);
    bollinger_bands.push(prices.compute_bollinger_band(num_avg, sigma));    
    price_histories.push(prices);
    for i in 2..args.len(){
        let (_, price_vec, _) = files[i-1].load(0);
        bollinger_bands.push(price_vec.compute_bollinger_band(num_avg, sigma));
        price_histories.push(price_vec);
    }
    let fee_rate = 0.25;
    let initial_capital = 100_000.0;
    let vec_duration: Vec<usize> = vec![250, 500, 1000, 2000, 2500];

    let mut ticker_fraction = Vec::new();
    for i in 1..args.len(){
        ticker_fraction.push(1.0 / (args.len() as f64));
    }

    let mut portfolio : RebalancePortfolioVecBollinger = RebalancePortfolioVecBollinger::new(initial_capital, &ticker_fraction, fee_rate, price_histories.clone(), bollinger_bands.clone());

    let mut cnt_strategy = 1;
    for duration in vec_duration.iter(){
        let tmp_rolling = portfolio.rolling_return(*duration);
        let (avg, _, _, perc) = analyze(&tmp_rolling, initial_capital);
        //let max_indices = p.remove_redundant(&p.sorting_final_capital(&tmp_rolling));
        let max_indices = tmp_rolling.sorting_final_capital().remove_redundant_from_maximum();
        let min_indices = tmp_rolling.sorting_final_capital().remove_redundant_from_minimum();
        let max_id = max_indices[0];
        let min_id = min_indices[0];
        let cap = tmp_rolling[max_id].unwrap();
        let cap_min = tmp_rolling[min_id].unwrap();
        
        //let profits : Vec<f64> = max_indices.iter().map(|id| tmp_rolling[*id].unwrap() / initial_capital).collect();                        
        //let mdds : Vec<f64> = max_indices.iter().map(|id| prices[id - *duration] / prev_maximum[id - *duration]).collect();
        //let dates : Vec<String> = max_indices.iter().map(|id| date_vec[id - *duration].clone()).collect();
        
    
        //file.write_sorted_vec(1, (cnt_strategy, 1), &format!("리밸런싱({}), 기간 {}", p.diff_ratio, *duration), &dates, &profits, &mdds);
        //cnt_strategy += 4;
        
        println!("리밸런싱(Bollinger Band), 기간 {} : 평균수익률 = {:.2}, 플러스수익확률 = {:.2}, 최대수익률 = {:.2}, 시작날짜 = {}, 최소수익률 = {:.2}, 시작날짜 = {}", duration, avg / initial_capital, perc, cap / initial_capital, date_vec[max_id - *duration], cap_min / initial_capital, date_vec[min_id - *duration]);    
    }

    let mut ticker_fraction = Vec::new();
    for i in 1..args.len(){
        ticker_fraction.push(1.0 / (args.len() as f64 - 1.0));
    }

    let mut buy_and_hold = BuyHoldPortfolioVec2::new(initial_capital, fee_rate, &ticker_fraction, price_histories.clone());
    for duration in vec_duration.iter(){
        let tmp_rolling = buy_and_hold.rolling_return(*duration);
        let max_indices = tmp_rolling.sorting_final_capital().remove_redundant_from_maximum();
        let min_indices = tmp_rolling.sorting_final_capital().remove_redundant_from_minimum();
                
        let (avg, _, _, perc) = analyze(&tmp_rolling, initial_capital);
        let max_id = max_indices[0];            
        let min_id = min_indices[0];
                
        let c = tmp_rolling[max_id].unwrap();
        let cap_min = tmp_rolling[min_id].unwrap();
        //let profits : Vec<f64> = max_indices.iter().map(|id| tmp_rolling[*id].unwrap() / initial_capital).collect();                        
        //let mdds : Vec<f64> = max_indices.iter().map(|id| prices[id - *duration] / prev_maximum[id - *duration]).collect();
        //let dates : Vec<String> = max_indices.iter().map(|id| date_vec[id - *duration].clone()).collect();

        //file.write_sorted_vec(1, (cnt_strategy, 1), &format!("보유, 기간 {}", *duration), &dates, &profits, &mdds);
        //cnt_strategy += 4;
               
        println!("보유, 기간 {} : 평균수익률 = {:.2}, 플러스수익확률 = {:.2}, 최대수익률 = {:.2}, 시작날짜 = {}, 최소수익률 = {:.2}, 시작날짜 = {}", duration, avg / initial_capital, perc, c / initial_capital, date_vec[max_id - *duration], cap_min / initial_capital, date_vec[min_id - *duration]);
    }
    //file.write_xlsx();
    let duration = start.elapsed();
    println!("실행 시간: {:.6}초 ({}ms)", duration.as_secs_f64(), duration.as_millis());
}

fn main_vec(){
    let start = Instant::now();

    let args: Vec<String> = env::args().collect();
    let mut files = Vec::new();
    let mut price_histories: Vec<StockPrices> = Vec::new();

    for i in 1..args.len(){
        files.push(StockFile::new(&format!("{}.xlsx", args[i])));        
    }
    let (date_vec, prices, prev_maximum) = files[0].load(0);
    price_histories.push(prices);

    for i in 2..args.len(){
        let (_, price_vec, _) = files[i-1].load(0);
        price_histories.push(price_vec);
    }
    let fee_rate = 0.25;
    let initial_capital = 100_000.0;
    let threshold = vec![0.1, 0.2, 0.3];
    let vec_duration: Vec<usize> = vec![250, 500, 1000, 2000, 2500];

    let mut ticker_fraction = Vec::new();
    for i in 1..args.len(){
        ticker_fraction.push(1.0 / (args.len() as f64));
    }

    let mut portfolio_vec :Vec<RebalancePortfolioVec> = threshold.iter().map(|x| RebalancePortfolioVec::new(initial_capital, &ticker_fraction, fee_rate, *x, price_histories[0].clone())).collect();
    
    let mut cnt_strategy = 1;
    for p in portfolio_vec.iter_mut(){
        for duration in vec_duration.iter(){
            let tmp_rolling = p.rolling_return(*duration);
            let (avg, _, _, perc) = analyze(&tmp_rolling, initial_capital);
            //let max_indices = p.remove_redundant(&p.sorting_final_capital(&tmp_rolling));
            let max_indices = tmp_rolling.sorting_final_capital().remove_redundant_from_maximum();
            let min_indices = tmp_rolling.sorting_final_capital().remove_redundant_from_minimum();
            let max_id = max_indices[0];
            let min_id = min_indices[0];
            let cap = tmp_rolling[max_id].unwrap();
            let cap_min = tmp_rolling[min_id].unwrap();
            
            println!("리밸런싱({}), 기간 {} : 평균수익률 = {:.2}, 플러스수익확률 = {:.2}, 최대수익률 = {:.2}, 시작날짜 = {}, 최소수익률 = {:.2}, 시작날짜 = {}", p.threshold, duration, avg / initial_capital, perc, cap / initial_capital, date_vec[max_id - *duration], cap_min / initial_capital, date_vec[min_id - *duration]);
        }
    }

    let mut ticker_fraction = Vec::new();
    for i in 1..args.len(){
        ticker_fraction.push(1.0 / (args.len() as f64 - 1.0));
    }

    let mut buy_and_hold = BuyHoldPortfolioVec::new(initial_capital, fee_rate, price_histories[0].clone());
    for duration in vec_duration.iter(){
        let tmp_rolling = buy_and_hold.rolling_return(*duration);
        let max_indices = tmp_rolling.sorting_final_capital().remove_redundant_from_maximum();
        let min_indices = tmp_rolling.sorting_final_capital().remove_redundant_from_minimum();
                
        let (avg, _, _, perc) = analyze(&tmp_rolling, initial_capital);
        let max_id = max_indices[0];            
        let min_id = min_indices[0];
                
        let c = tmp_rolling[max_id].unwrap();
        let cap_min = tmp_rolling[min_id].unwrap();
        println!("보유, 기간 {} : 평균수익률 = {:.2}, 플러스수익확률 = {:.2}, 최대수익률 = {:.2}, 시작날짜 = {}, 최소수익률 = {:.2}, 시작날짜 = {}", duration, avg / initial_capital, perc, c / initial_capital, date_vec[max_id - *duration], cap_min / initial_capital, date_vec[min_id - *duration]);
    }
    //file.write_xlsx();
    let duration = start.elapsed();
    println!("실행 시간: {:.6}초 ({}ms)", duration.as_secs_f64(), duration.as_millis());

}
fn main_original(){
    let start = Instant::now();

    let args: Vec<String> = env::args().collect();
    let file_name = format!("{}.xlsx", args[1]);
    let mut file = StockFile::new(&file_name);
    let (date_vec, prices, prev_maximum) = file.load(0);
    let fee_rate = 0.25;
    let initial_capital = 100_000.0;
    let threshold = vec![0.1, 0.2, 0.3];
    let vec_duration: Vec<usize> = vec![250, 500, 1000, 2000, 2500];

    let mut portfolio_vec :Vec<RebalancePortfolio> = threshold.iter().map(|x| RebalancePortfolio::new(initial_capital, *x, 0.5, fee_rate, prices.clone())).collect();
    let mut cnt_strategy = 1;
    for p in portfolio_vec.iter_mut(){
        for duration in vec_duration.iter(){
            let tmp_rolling = p.rolling_return(*duration);
            let (avg, _, _, perc) = analyze(&tmp_rolling, initial_capital);
            //let max_indices = p.remove_redundant(&p.sorting_final_capital(&tmp_rolling));
            let max_indices = tmp_rolling.sorting_final_capital().remove_redundant_from_maximum();
            let min_indices = tmp_rolling.sorting_final_capital().remove_redundant_from_minimum();
            let max_id = max_indices[0];
            let min_id = min_indices[0];
            let cap = tmp_rolling[max_id].unwrap();
            let cap_min = tmp_rolling[min_id].unwrap();
            
            //let profits : Vec<f64> = max_indices.iter().map(|id| tmp_rolling[*id].unwrap() / initial_capital).collect();                        
            //let mdds : Vec<f64> = max_indices.iter().map(|id| prices[id - *duration] / prev_maximum[id - *duration]).collect();
            //let dates : Vec<String> = max_indices.iter().map(|id| date_vec[id - *duration].clone()).collect();
            
            //file.write_sorted_vec(1, (cnt_strategy, 1), &format!("리밸런싱({}), 기간 {}", p.diff_ratio, *duration), &dates, &profits, &mdds);
            //cnt_strategy += 4;
            
            println!("리밸런싱({}), 기간 {} : 평균수익률 = {:.2}, 플러스수익확률 = {:.2}, 최대수익률 = {:.2}, 전고점대비 = {:.2}, 시작날짜 = {}, 최소수익률 = {:.2}, 전고점대비 = {:.2}, 시작날짜 = {}", p.threshold, duration, avg / initial_capital, perc, cap / initial_capital, prices[max_id - *duration] / prev_maximum[max_id - *duration], date_vec[max_id - *duration], cap_min / initial_capital, prices[min_id - *duration] / prev_maximum[min_id - *duration], date_vec[min_id - *duration]);
        }
    }

    let mut buy_and_hold = BuyHoldPortfolio::new(initial_capital, fee_rate, prices.clone());
    for duration in vec_duration.iter(){
        let tmp_rolling = buy_and_hold.rolling_return(*duration);
        let max_indices = tmp_rolling.sorting_final_capital().remove_redundant_from_maximum();
        let min_indices = tmp_rolling.sorting_final_capital().remove_redundant_from_minimum();
                
        let (avg, _, _, perc) = analyze(&tmp_rolling, initial_capital);
        let max_id = max_indices[0];            
        let min_id = min_indices[0];
                
        let c = tmp_rolling[max_id].unwrap();
        let cap_min = tmp_rolling[min_id].unwrap();
        //let profits : Vec<f64> = max_indices.iter().map(|id| tmp_rolling[*id].unwrap() / initial_capital).collect();                        
        //let mdds : Vec<f64> = max_indices.iter().map(|id| prices[id - *duration] / prev_maximum[id - *duration]).collect();
        //let dates : Vec<String> = max_indices.iter().map(|id| date_vec[id - *duration].clone()).collect();

        //file.write_sorted_vec(1, (cnt_strategy, 1), &format!("보유, 기간 {}", *duration), &dates, &profits, &mdds);
        //cnt_strategy += 4;
               
        println!("보유, 기간 {} : 평균수익률 = {:.2}, 플러스수익확률 = {:.2}, 최대수익률 = {:.2}, 전고점대비 = {:.2}, 시작날짜 = {}, 최소수익률 = {:.2}, 전고점대비 = {:.2}, 시작날짜 = {}", duration, avg / initial_capital, perc, c / initial_capital, prices[max_id - *duration] / prev_maximum[max_id - *duration], date_vec[max_id - *duration], cap_min / initial_capital, prices[min_id - *duration] / prev_maximum[min_id - *duration], date_vec[min_id - *duration]);
    }
    //file.write_xlsx();
    let duration = start.elapsed();
    println!("실행 시간: {:.6}초 ({}ms)", duration.as_secs_f64(), duration.as_millis());

}
fn main(){
    main_hashmap();
    main_original();
    main_vec();
    main_vec2();
    //main_bollinger();
}