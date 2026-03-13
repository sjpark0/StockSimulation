use umya_spreadsheet::*;
use std::path::Path;
use std::env;
mod stock_file;
/*
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let file_name = format!("{}.xlsx", args[1]);
    let file_path = Path::new(&file_name);

    let mut book = reader::xlsx::read(file_path)?;
    let sheet = book.get_sheet_mut(&0).unwrap();
    let highest_row = sheet.get_highest_row();

    sheet.get_cell_mut((3, 1)).set_value("일일 수익률");
    let mut prev_price: Option<f64> = None;

    for row in 3..=highest_row {
        let price_str = sheet.get_value((2, row));

        if let Ok(current_price) = price_str.parse::<f64>(){
            let cell = sheet.get_cell_mut((3, row));
            if let Some(yesterday_price) = prev_price{
                let daily_return = (current_price - yesterday_price) / yesterday_price;
                cell.set_value_number(daily_return);
                
                // 백분율(%) 포맷 스타일을 셀에 적용합니다 (예: 15.34%).
                let mut style = Style::default();
                style.get_number_format_mut().set_format_code("0.00%");
                cell.set_style(style);
            }
            else{
                cell.set_value("-");
            }
            prev_price = Some(current_price);
        }
        
    }
    writer::xlsx::write(&book, file_path)?;

    Ok(())
}
*/
fn main(){
    let args: Vec<String> = env::args().collect();
    let file_name = format!("{}.xlsx", args[1]);
    let prices = stock_file::load_excel_file(&file_name);
    for p in &prices{
        println!("{}", p);
    }
}