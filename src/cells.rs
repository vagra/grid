use std::{ops::{Index, IndexMut}};
use crate::pool::*;


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

impl Cell {

    pub fn new(head:u16) -> Self {

        Self {
            head: head,
        }
    }
}


#[derive(Debug, Clone)]
pub struct Cols(Vec<Cell>);


impl Default for Cols {

    fn default() -> Self {
        
        Self(Vec::default())
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

    pub fn new(cols:u16) -> Self {

        let mut vec:Vec<Cell> = Vec::new();

        for _ in 0..cols {
            let cell = Cell::default();

            vec.push(cell);
        }

        Self(vec)
    }

    pub fn len(&self) -> u16 {
        
        self.0.len() as u16
    }
}


#[derive(Debug)]
pub struct Rows(Vec<Cols>);


impl Default for Rows {

    fn default() -> Self {
        
        Self(Vec::default())
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

    pub fn new(rows: u16, cols: u16) -> Self {

        let mut vec: Vec<Cols> = Vec::new();

        for _ in 0..rows {
            let row = Cols::new(cols);

            vec.push(row);
        }

        Self(vec)
    }

    pub fn len(&self) -> u16 {
        
        self.0.len() as u16
    }
}
