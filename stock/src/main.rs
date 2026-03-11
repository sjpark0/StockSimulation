use yahoo_finance_api as yahoo;
use time::{OffsetDateTime, Duration};
use tokio;

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
        (*self / initial_value) * 100.0 - 100.0
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let initial_capital = 100_000.0;
    let threshold = vec![0.1, 0.2, 0.3, 0.4, 0.5];
    
    let provider = yahoo::YahooConnector::new()?;

    let end = OffsetDateTime::now_utc();
    let start = end - Duration::days(10000);

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

    // 메모리에 로드된 배열을 순회하며 백테스트 진행
    print!("\tBuy and hold");
    for t in &threshold{
        print!("\t\tRebalancing {}", t);
    }
    println!();

    print!("날짜");
    for _ in &threshold{
        print!("\tPrice\tProfit");
    }
    println!();
    
    for quote in &quotes {
        // 타임스탬프를 읽기 쉬운 날짜 문자열로 변환 (간단히 표시)
        let date = OffsetDateTime::from_unix_timestamp(quote.timestamp as i64)
            .unwrap()
            .date()
            .to_string();
            
        let current_price = quote.close;
        let buy_and_hold_value = initial_qty * current_price + initial_cash;
        print!("{}\t{:.2}\t{:.2}%\t", date, buy_and_hold_value, buy_and_hold_value.get_profit_rate(initial_capital));
        
        for tester in backtester.iter_mut(){
            tester.process_price(current_price);
            let current_value = tester.get_total_value(current_price);
            print!("{:.2}\t{:.2}%\t", current_value, current_value.get_profit_rate(initial_capital));        
        }
        println!();     
    }

    // 최종 결과 출력
    let final_price = quotes.last().unwrap().close;

    let buy_and_hold_value = initial_qty * final_price + initial_cash;
    print!("\t{:.2}\t{:.2}%\t",buy_and_hold_value, buy_and_hold_value.get_profit_rate(initial_capital));
        
    for tester in backtester.iter_mut(){
        tester.process_price(final_price);
        let current_value = tester.get_total_value(final_price);
        print!("{:.2}\t{:.2}%\t", current_value, current_value.get_profit_rate(initial_capital));        
    }
    println!();
    Ok(())

}
