use umya_spreadsheet::*;
use umya_spreadsheet::helper::coordinate::coordinate_from_index;

use std::path::Path;

pub struct StockFile{
    book : Spreadsheet,
    file_path : String,
}

impl StockFile{
    pub fn new(file_name : &str) -> Self{
        let file_path = Path::new(&file_name);
        Self{ book : reader::xlsx::read(file_path).unwrap(),  file_path : file_name.to_string()}    
    }
    pub fn load(&mut self, sheet_idx : usize) -> (Vec<String>, Vec<f64>, Vec<f64>){
        let sheet = self.book.get_sheet_mut(&sheet_idx).unwrap();
        let highest_row = sheet.get_highest_row();

        let mut res_price = Vec::new();
        let mut res_date = Vec::new();
        let mut res_pre_maximum = Vec::new();
        for row in 3..=highest_row {
            let date = sheet.get_value((1, row));
            let price_str = sheet.get_value((2, row));

            if let Ok(current_price) = price_str.parse::<f64>(){
                res_price.push(current_price);
                res_date.push(date);
                if let Some(val) = res_pre_maximum.last(){
                    res_pre_maximum.push(current_price.max(*val));
                } 
                else{
                    res_pre_maximum.push(current_price);
                }
            }        
        }

        (res_date, res_price, res_pre_maximum)    
    }
    pub fn write_xlsx(&mut self){
        let file_path = Path::new(&self.file_path);
        writer::xlsx::write(&self.book, &file_path).unwrap();
    }
    pub fn write_sorted_vec(&mut self, sheet_idx : usize, start_coords: (u32, u32), header : &str, dates : &[String], profit : &[f64], mdd : &[f64]){
        let sheet_res = self.book.get_sheet_mut(&sheet_idx);
        let sheet = if sheet_res.is_err(){
            self.book.new_sheet("Sheet 2").unwrap()
        } else{
            sheet_res.unwrap()
        };

        let start_cell = coordinate_from_index(&start_coords.0, &start_coords.1);
        let end_cell = coordinate_from_index(&(start_coords.0 + 2), &start_coords.1);
        sheet.add_merge_cells(format!("{}:{}", start_cell, end_cell));

        let cell = sheet.get_cell_mut(start_cell);
        cell.set_value_string(header.to_string());
        
        let cell = sheet.get_cell_mut((start_coords.0, start_coords.1 + 1));
        cell.set_value_string("시작날짜".to_string());

        let cell = sheet.get_cell_mut((start_coords.0 + 1, start_coords.1 + 1));
        cell.set_value_string("수익률".to_string());
        
        let cell = sheet.get_cell_mut((start_coords.0 + 2, start_coords.1 + 1));
        cell.set_value_string("고점대비".to_string());

        for (i, date) in dates.iter().enumerate(){
            let cell = sheet.get_cell_mut((start_coords.0, start_coords.1 + 2 + i as u32));
            cell.set_value_string(date.clone());
            let cell = sheet.get_cell_mut((start_coords.0 + 1, start_coords.1 + 2 + i as u32));
            cell.set_value_number(profit[i]);
            let cell = sheet.get_cell_mut((start_coords.0 + 2, start_coords.1 + 2 + i as u32));
            cell.set_value_number(mdd[i]);        
        }
    }
    pub fn merge_cells(&mut self, sheet_idx : usize, start_coords: (u32, u32), end_coords : (u32, u32)){
        let sheet = self.book.get_sheet_mut(&sheet_idx).unwrap();
        let start_cell = coordinate_from_index(&start_coords.0, &start_coords.1);
        let end_cell = coordinate_from_index(&end_coords.0, &end_coords.1);
        sheet.add_merge_cells(format!("{}:{}", start_cell, end_cell));
    }
    pub fn write_header(&mut self, sheet_idx : usize, coords : (u32, u32), header : &str){
        let sheet = self.book.get_sheet_mut(&sheet_idx).unwrap();
        let cell = sheet.get_cell_mut(coords);
        cell.set_value_string(header);
        
    }
    
    pub fn write_capital(&mut self, sheet_idx : usize, coords : (u32, u32), number : f64){
        let sheet = self.book.get_sheet_mut(&sheet_idx).unwrap();
        let cell = sheet.get_cell_mut(coords);
        cell.set_value_number(number);
    }
    pub fn write_rate(&mut self, sheet_idx : usize, coords : (u32, u32), number : f64){
        let sheet = self.book.get_sheet_mut(&sheet_idx).unwrap();
        let cell = sheet.get_cell_mut(coords);
        cell.set_value_number(number);
        let mut style = Style::default();
        style.get_number_format_mut().set_format_code("0.00%");
        cell.set_style(style);
    }
    pub fn write_number(&mut self, sheet_idx : usize, coords : (u32, u32), number : i32){
        let sheet = self.book.get_sheet_mut(&sheet_idx).unwrap();
        let cell = sheet.get_cell_mut(coords);
        cell.set_value_number(number);        
    }
}
