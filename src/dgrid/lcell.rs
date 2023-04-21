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