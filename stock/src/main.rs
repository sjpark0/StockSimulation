use yahoo_finance_api as yahoo;
use time::{OffsetDateTime, Duration};
use tokio;
use rand::Rng;
use rust_xlsxwriter::{Format, FormatAlign, Workbook};

struct Portfolio{
    cash: f64,
    stock_qty: u32,
    threshold: f64,
}

impl Portfolio{
    fn new(initial_capital: f64, initial_price: f64, threshold: f64) -> Self{
        let half_capital = initial_capital / 2.0;
        let qty = (half_capital / initial_price).round() as u32;
        let cash = initial_capital - (qty as f64) * initial_price;
        Self { cash: cash, stock_qty: qty, threshold: threshold}
    }
    fn process_price(&mut self, current_price: f64){
        let stock_value = (self.stock_qty as f64) * current_price;
        let total_value = stock_value + self.cash;

        let stock_weight = stock_value / total_value;
        let cash_weight = self.cash / total_value;

        if(stock_weight - cash_weight).abs() >= self.threshold{
            self.rebalance(current_price, total_value);
        }
    }
    fn rebalance(&mut self, current_price: f64, total_value: f64){
        let half_capital = total_value / 2.0;
        self.stock_qty = (half_capital / current_price).round() as u32;
        self.cash = total_value - (self.stock_qty as f64) * current_price;
    }
    fn get_total_value(&self, current_price: f64) -> f64{
        self.cash + (self.stock_qty as f64) * current_price
    }
}
trait Rate {
    fn get_profit_rate(&self, initial_value : f64) -> f64;
}
impl Rate for f64{
    fn get_profit_rate(&self, initial_value : f64) -> f64{
        (*self / initial_value) - 1.0
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let initial_capital = 100_000.0;
    let threshold = vec![0.1, 0.2, 0.3, 0.4, 0.5];
    
    let provider = yahoo::YahooConnector::new()?;

    let end = OffsetDateTime::now_utc();
    //let start = end - Duration::days(rand::thread_rng().gen_range(1, 4000));
    let start = end - Duration::days(1000000);
    // BTC-USD(비트코인)의 1일봉(1d) 데이터를 메모리로 직접 가져옴
    let response = provider.get_quote_history("SOXL", start, end).await?;
    let quotes = response.quotes()?;

    if quotes.is_empty() {
        println!("데이터를 불러오지 못했습니다.");
        return Ok(());
    }

    // 첫 날의 종가를 기준으로 초기 포트폴리오 세팅 (예: 10만 달러 시작, 10% 임계치)
    let initial_price = quotes.first().unwrap().close;
    let initial_qty = (initial_capital / initial_price).round();
    let initial_cash = initial_capital - initial_price * initial_qty;

    let mut backtester: Vec<Portfolio> = vec![];
    for t in &threshold{
        backtester.push(Portfolio::new(initial_capital, initial_price, *t));
    }

    
    println!("--- 백테스트 시작 (초기 자산: $100,000) ---");

    // 1. 새로운 엑셀 워크북(파일)과 시트 생성
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // 2. 글자를 한가운데로 예쁘게 모아줄 정렬 포맷 만들기
    let center_format = Format::new()
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter);
    let percent_format = Format::new().set_num_format("0.00%");

    worksheet.merge_range(0, 0, 1, 0, "날짜", &center_format)?;
    worksheet.merge_range(0, 1, 0, 2, "단순보유", &center_format)?;
    worksheet.write_string(1, 1, "가격")?;
    worksheet.write_string(1, 2, "수익률")?;
    
    for (i, t) in threshold.iter().enumerate(){
        worksheet.merge_range(0, 3+2*(i as u16), 0, 4+2*(i as u16), &format!("리밸런싱({}%) 자산", t * 100.0), &center_format)?;        
        worksheet.write_string(1, 3+2*(i as u16), "가격")?;
        worksheet.write_string(1, 4+2*(i as u16), "수익률")?;
    }
    
    let mut i = 2;
    let mut val_vec = vec![0.0;threshold.len() + 2];
    let mut acc_win = vec![0;threshold.len() + 2];
    for quote in quotes.iter() {
        // 타임스탬프를 읽기 쉬운 날짜 문자열로 변환 (간단히 표시)
        let date = OffsetDateTime::from_unix_timestamp(quote.timestamp as i64)
            .unwrap()
            .date()
            .to_string();
            
        let current_price = quote.close;
        let buy_and_hold_value = initial_qty * current_price + initial_cash;
        worksheet.write_string(i as u32, 0, &date)?;
        worksheet.write_number(i as u32, 1, buy_and_hold_value)?;
        worksheet.write_number_with_format(i as u32, 2, buy_and_hold_value.get_profit_rate(initial_capital), &percent_format)?;
        val_vec[threshold.len()] = buy_and_hold_value;
        val_vec[threshold.len() + 1] = initial_capital;
        
        for (j, tester) in backtester.iter_mut().enumerate(){
            tester.process_price(current_price);
            let current_value = tester.get_total_value(current_price);
            val_vec[j] = current_value;
            worksheet.write_number(i as u32, 3+2*(j as u16), current_value)?;
            worksheet.write_number_with_format(i as u32, 4+2*(j as u16), current_value.get_profit_rate(initial_capital), &percent_format)?;
        }
        let (idx, max_val) = val_vec.iter().enumerate().fold((threshold.len() + 1, val_vec[threshold.len() + 1]), |(acc_id, acc_val), (id, &val)| if val > acc_val { (id, val)} else {(acc_id, acc_val)});
        acc_win[idx] += 1;
        worksheet.write_number(i as u32, 3+2*(threshold.len() as u16), idx as u16)?;
        
        i += 1;
    }

    // 최종 결과 출력
    let final_price = quotes.last().unwrap().close;

    let buy_and_hold_value = initial_qty * final_price + initial_cash;

    worksheet.write_number(i as u32, 1, buy_and_hold_value)?;
    worksheet.write_number_with_format(i as u32, 2, buy_and_hold_value.get_profit_rate(initial_capital), &percent_format)?;
        
    for (j, tester) in backtester.iter_mut().enumerate(){
        tester.process_price(final_price);
        let current_value = tester.get_total_value(final_price);

        worksheet.write_number(i as u32, 3+2*(j as u16), current_value)?;
        worksheet.write_number_with_format(i as u32, 4+2*(j as u16), current_value.get_profit_rate(initial_capital), &percent_format)?;
    }
    i+= 1;
    for idx in 0..threshold.len(){
        worksheet.write_string(i as u32, 0, &format!("리밸런싱({}%) 자산", threshold[idx] * 100.0))?;        
        worksheet.write_number(i as u32, 1, acc_win[idx])?;        
        i += 1;
    }
    
    worksheet.write_string(i as u32, 0, "단순보유")?;        
    worksheet.write_number(i as u32, 1, acc_win[threshold.len()])?;        
    i += 1;
    
    worksheet.write_string(i as u32, 0, "현금")?;        
    worksheet.write_number(i as u32, 1, acc_win[threshold.len() + 1])?;        
    
    workbook.save("backtest_report.xlsx")?;

    for idx in 0..threshold.len(){
        println!("리밸런싱({})자산 : {}", threshold[idx] * 100.0, acc_win[idx]);
    }
    println!("단순보유 : {}", acc_win[threshold.len()]);
    println!("현금 : {}", acc_win[threshold.len()+1]);
    
    
    Ok(())

}
