use yahoo_finance_api as yahoo;
use time::{OffsetDateTime, Duration};
use tokio;
use rust_xlsxwriter::{Format, FormatAlign, Workbook};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let provider = yahoo::YahooConnector::new()?;

    let end = OffsetDateTime::now_utc();
    let start = OffsetDateTime::UNIX_EPOCH;

    // BTC-USD(비트코인)의 1일봉(1d) 데이터를 메모리로 직접 가져옴
    let response = provider.get_quote_history(&args[1], start, end).await?;
    let quotes = response.quotes()?;

    if quotes.is_empty() {
        println!("데이터를 불러오지 못했습니다.");
        return Ok(());
    }
    
    // 1. 새로운 엑셀 워크북(파일)과 시트 생성
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // 2. 글자를 한가운데로 예쁘게 모아줄 정렬 포맷 만들기
    let center_format = Format::new().set_align(FormatAlign::Center).set_align(FormatAlign::VerticalCenter);
    
    worksheet.merge_range(0, 0, 1, 0, "날짜", &center_format)?;
    worksheet.merge_range(0, 1, 1, 1, "가격", &center_format)?;
        
    let mut i = 2;
    for quote in quotes.iter() {
        // 타임스탬프를 읽기 쉬운 날짜 문자열로 변환 (간단히 표시)
        let date = OffsetDateTime::from_unix_timestamp(quote.timestamp as i64).unwrap().date().to_string();
            
        let current_price = quote.close;
        
        worksheet.write_string(i as u32, 0, &date)?;
        worksheet.write_number(i as u32, 1, current_price)?;        
        i += 1;
    }
    
    workbook.save(format!("{}.xlsx", &args[1]))?;
    
    Ok(())

}
