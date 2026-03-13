use umya_spreadsheet::*;
use std::path::Path;

pub fn load_excel_file(file_name : &str) -> Vec<f64>{
    let file_path = Path::new(&file_name);
    let mut book = reader::xlsx::read(file_path).unwrap();
    let sheet = book.get_sheet_mut(&0).unwrap();
    let highest_row = sheet.get_highest_row();

    let mut prev_price: Option<f64> = None;
    let mut res = Vec::new();
    for row in 3..=highest_row {
        let price_str = sheet.get_value((2, row));

        if let Ok(current_price) = price_str.parse::<f64>(){
            res.push(current_price);
        }        
    }

    res
}