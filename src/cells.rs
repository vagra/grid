use std::{ops::{Index, IndexMut}};
use crate::{pool::*, ugrid::*};


#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub head: u16,
}

impl Default for Cell {
    fn default() -> Self {

        Self {
            head: INVALID,
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Cols ([Cell; COLS as usize]);


impl Default for Cols {

    fn default() -> Self {
        
        Self([Cell::default(); COLS as usize])
    }
}

impl Index<u16> for Cols {
    type Output = Cell;

    fn index(&self, index: u16) -> &Self::Output {
        
        &self.0[index as usize]
    }
}

impl IndexMut<u16> for Cols {

    fn index_mut(&mut self, index: u16) -> &mut Self::Output {

        &mut self.0[index as usize]
    }
}


impl Cols {

    pub fn len(&self) -> u16 {
        
        self.0.len() as u16
    }
}


#[derive(Debug)]
pub struct Rows([Cols; ROWS as usize]);


impl Default for Rows {

    fn default() -> Self {
        
        Self([Cols::default(); ROWS as usize])
    }
}

impl Index<u16> for Rows {
    type Output = Cols;

    fn index(&self, index: u16) -> &Self::Output {
        
        &self.0[index as usize]
    }
}

impl IndexMut<u16> for Rows {

    fn index_mut(&mut self, index: u16) -> &mut Self::Output {

        &mut self.0[index as usize]
    }
}

impl Rows {

    pub fn len(&self) -> u16 {
        
        self.0.len() as u16
    }
}
