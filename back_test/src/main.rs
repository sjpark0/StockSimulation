use std::env;
mod stock_file;
mod portfolio;

use portfolio::Backtester;
fn maimum_profit(capital_profit_vec : &[Option<(f64, f64, f64)>]) -> (f64, f64, f64){
    let (mut max_capital, mut max_profit, mut max_mdd) = (0.0, 0.0, 0.0);
    
    for v in capital_profit_vec.iter(){
        if let Some((c, p, m)) = v{
            if max_capital < *c{
                (max_capital, max_profit, max_mdd) = (*c, *p, *m);
            }
        }
    }

    (max_capital, max_profit, max_mdd)
    
}
fn analyze(capital_profit_vec : &[Option<(f64, f64, f64)>], initial_price : f64) -> (f64, f64, u32, f64){
    let mut cnt = 0.0;
    for v in capital_profit_vec.iter(){
        if v.is_some(){
            cnt += 1.0;
        }
    }
    let mut avg_capital = 0.0;
    for v in capital_profit_vec.iter(){
        if let Some((c, _, _)) = v{
            avg_capital += c / cnt;
        }
    }
    let mut std_capital = 0.0;
    for v in capital_profit_vec.iter(){
        if let Some((c, _, _)) = v{
            std_capital += (avg_capital - c) * (avg_capital - c) / cnt;
        }
    }
    
    let mut num_win = 0;
    for v in capital_profit_vec.iter(){
        if let Some((c, _, _)) = v{
            if *c > initial_price{
                num_win += 1;
            }
        }
    }
    
    (avg_capital, std_capital.sqrt(), num_win, (num_win as f64) / (cnt as f64))
}
fn main(){
    let args: Vec<String> = env::args().collect();
    let file_name = format!("{}.xlsx", args[1]);
    let prices = stock_file::load_excel_file(&file_name);
    let fee_rate = 0.25;
    let initial_capital = 100_000.0;
    let threshold = vec![0.1, 0.2, 0.3, 0.4, 0.5];
    let vec_duration: Vec<usize> = vec![21, 60, 200, 400];
    
    for t in threshold.iter(){
        for duration in vec_duration.iter(){
            let tmp_rolling = t.rolling_return(&prices, initial_capital, 0.5, *duration, fee_rate);
            let (avg, std, num_win, perc) = analyze(&tmp_rolling, initial_capital);
            let (max_cap, max_profit, max_mdd) = maimum_profit(&tmp_rolling);
            println!("리밸런싱({}), 기간 {} : 평균 = {:.2}, 평균수익률 = {:.2}, 표준편차 = {:.2}, 플러스수익횟수 = {}, 플러스수익확률 = {:.2}, 최대수익금 = {:.2}, 최대수익률 = {:.2}, 최대수익때mdd = {:.2}", t, duration, avg, avg / initial_capital, std, num_win, perc, max_cap, max_profit, max_mdd);
        }
    }
    for duration in vec_duration.iter(){
        let tmp_rolling = &(1.0).rolling_return(&prices, initial_capital, 1.0, *duration, fee_rate);
        let (avg, std, num_win, perc) = analyze(&tmp_rolling, initial_capital);
        let (max_cap, max_profit, max_mdd) = maimum_profit(&tmp_rolling);
        println!("보유, 기간 {} : 평균 = {:.2}, 평균수익률 = {:.2}, 표준편차 = {:.2}, 플러스수익횟수 = {}, 플러스수익확률 = {:.2}, 최대수익금 = {:.2}, 최대수익률 = {:.2}, 최대수익때mdd = {:.2}", duration, avg, avg / initial_capital, std, num_win, perc, max_cap, max_profit, max_mdd);
    }
}