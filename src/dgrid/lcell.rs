use std::{ops::{Index, IndexMut}};
use crate::*;



#[derive(Debug, Clone)]
pub struct LCell {
    pub head: u16,

    pub l: i16,
    pub t: i16,
    pub r: i16,
    pub b: i16,
}


impl Default for LCell {

    fn default() -> Self {
        
        Self {
            head: INVALID,
            l: 0,
            t: 0,
            r: 0,
            b: 0,
        }
    }
}



#[derive(Debug)]
pub struct LCells(Vec<LCell>);


impl Default for LCells {

    fn default() -> Self {
        
        Self(Vec::default())
    }
}

impl Index<u16> for LCells {
    type Output = LCell;

    fn index(&self, index: u16) -> &Self::Output {
        
        &self.0[index as usize]
    }
}

impl IndexMut<u16> for LCells {

    fn index_mut(&mut self, index: u16) -> &mut Self::Output {

        &mut self.0[index as usize]
    }
}

impl LCells {

    pub fn len(&self) -> u16 {
        
        self.0.len() as u16
    }
}
