use std::ops::{Deref, DerefMut};
use std::collections::VecDeque;
pub struct CapitalReturns(pub Vec<Option<f64>>);
pub struct PortfolidIndices(pub Vec<usize>);

enum Asset{
    CASH,
    STOCK(String, u32, f64),
}

impl Deref for CapitalReturns {
    type Target = Vec<Option<f64>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CapitalReturns {
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

impl CapitalReturns {
    pub fn sorting_final_capital(&self) -> PortfolidIndices{
        
        let mut indices : PortfolidIndices = PortfolidIndices(self.iter().enumerate().filter_map(|(i, val)| if val.is_some() {Some(i)} else {None}).collect());

        indices.sort_unstable_by(|&i, &j| {
            let a = self[i].unwrap();
            let b = self[j].unwrap();
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
