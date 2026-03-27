use std::ops::{Deref, DerefMut};
use std::collections::VecDeque;
use std::collections::HashMap;

pub struct CapitalReturns(pub Vec<Option<(f64, f64)>>);
pub struct PortfolidIndices(pub Vec<usize>);

#[derive(Clone)]
pub struct StockPrices(pub Vec<f64>);

#[derive(Clone)]
pub struct BollingerBand(pub Vec<Option<(f64, f64)>>);

#[derive(Debug)]
pub struct Assets(pub Vec<(f64, f64)>);


pub struct AssetsHashmap(pub HashMap<String, (f64, f64)>);

impl Deref for CapitalReturns {
    type Target = Vec<Option<(f64, f64)>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CapitalReturns {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for StockPrices {
    type Target = Vec<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StockPrices {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for PortfolidIndices {
    type Target = Vec<usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PortfolidIndices {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Assets {
    type Target = Vec<(f64, f64)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Assets {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for AssetsHashmap {
    type Target = HashMap<String, (f64, f64)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AssetsHashmap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl CapitalReturns {
    pub fn sorting_final_capital(&self) -> PortfolidIndices{
        
        let mut indices : PortfolidIndices = PortfolidIndices(self.iter().enumerate().filter_map(|(i, val)| if val.is_some() {Some(i)} else {None}).collect());

        indices.sort_unstable_by(|&i, &j| {
            let a = self[i].unwrap().0;
            let b = self[j].unwrap().0;
            b.total_cmp(&a)
        });
        indices
    }
}

impl PortfolidIndices{
    pub fn remove_redundant_from_maximum(&self) -> PortfolidIndices{
        let mut res_idx = VecDeque::with_capacity(self.len());
        res_idx.extend(self.iter());
        let mut res_idx1: PortfolidIndices = PortfolidIndices(Vec::new());
        while let Some(num) = res_idx.pop_front(){
            res_idx.retain(|&x| x > num + 21 || x < num - 21);    
            res_idx1.push(num);        
        }    
        res_idx1
    }

    pub fn remove_redundant_from_minimum(&self) -> PortfolidIndices{
        let mut res_idx = VecDeque::with_capacity(self.len());
        res_idx.extend(self.iter());
        let mut res_idx1: PortfolidIndices = PortfolidIndices(Vec::new());
        while let Some(num) = res_idx.pop_back(){
            res_idx.retain(|&x| x > num + 21 || x < num - 21);    
            res_idx1.push(num);        
        }    
        res_idx1
    }
}

impl StockPrices{
    pub fn compute_bollinger_band(&self, num_avg : usize, sigma : f64) -> BollingerBand{
        let length = self.len();
        let mut moving_avg = (0..num_avg).fold(0.0, |acc, idx| acc + self[idx] / (num_avg as f64));
        let avg_initial = moving_avg;

        let avgs : Vec<f64> = (num_avg..length).map(|idx|{
            moving_avg = moving_avg - self[idx - num_avg] / (num_avg as f64) + self[idx] / (num_avg as f64);
            moving_avg
        }).collect();

        let std_initial = (0..num_avg).fold(0.0, |acc, idx| acc + (self[num_avg - 1 - idx] - avg_initial) * (self[num_avg - 1 - idx] - avg_initial) / (num_avg as f64)).sqrt();
        let stds : Vec<f64> = avgs.iter().enumerate().map(|(current_idx, avg)|{
            let moving_std = (0..num_avg).fold(0.0, |acc, idx| acc + (self[current_idx + num_avg - idx] - avg) * (self[current_idx + num_avg - idx] - avg) / (num_avg as f64)).sqrt();
            moving_std
        }).collect(); 
        let mut band  = BollingerBand(vec![None;num_avg]);
        band.0.push(Some((avg_initial + sigma * std_initial, avg_initial - sigma * std_initial)));  
        let bollinger : Vec<Option<(f64, f64)>> = stds.iter().zip(avgs.iter()).map(|(s, a)| Some((*a + sigma * *s, *a - sigma * *s))).collect();
        band.0.extend(bollinger);
        band        
    }
    pub fn is_not_range(&self, bollinger_band : &BollingerBand, date_idx : usize) -> bool{
        if let Some((upper, lower)) = bollinger_band.0[date_idx]{
            self[date_idx] < lower || self[date_idx] > upper
        }
        else{
            false
        }
    }
}