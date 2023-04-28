use std::{ops::{Index, IndexMut}};

use crate::{CellComm, CellSpec};


#[derive(Debug, Clone)]
pub struct Cols<T:CellComm+CellSpec>(Vec<T>);


impl<T:CellComm+CellSpec + Default>
Default for Cols<T> {

    fn default() -> Self {
        
        Self(Vec::default())
    }
}

impl<T:CellComm+CellSpec>
Index<u16> for Cols<T> {
    type Output = T;

    fn index(&self, index: u16) -> &Self::Output {
        
        &self.0[index as usize]
    }
}

impl<T:CellComm+CellSpec>
IndexMut<u16> for Cols<T> {

    fn index_mut(&mut self, index: u16) -> &mut Self::Output {

        &mut self.0[index as usize]
    }
}


impl<T:CellComm+CellSpec + Default>
Cols<T> {

    pub fn new(cols:u16) -> Self {

        let mut vec:Vec<T> = Vec::new();

        for _ in 0..cols {

            vec.push(T::default());
        }

        Self(vec)
    }

    pub fn len(&self) -> u16 {
        
        self.0.len() as u16
    }
}


#[derive(Debug)]
pub struct Cells<T:CellComm+CellSpec>(Vec<Cols<T>>);


impl<T:CellComm+CellSpec + Default>
Default for Cells<T> {

    fn default() -> Self {
        
        Self(Vec::default())
    }
}

impl<T:CellComm+CellSpec>
Index<u16> for Cells<T> {
    type Output = Cols<T>;

    fn index(&self, index: u16) -> &Self::Output {
        
        &self.0[index as usize]
    }
}

impl<T:CellComm+CellSpec>
IndexMut<u16> for Cells<T> {

    fn index_mut(&mut self, index: u16) -> &mut Self::Output {

        &mut self.0[index as usize]
    }
}

impl<T:CellComm+CellSpec + Default>
Cells<T> {

    pub fn new(rows: u16, cols: u16) -> Self {

        let mut vec: Vec<Cols<T>> = Vec::new();

        for _ in 0..rows {

            vec.push(Cols::new(cols));
        }

        Self(vec)
    }

    pub fn clear(&mut self) {

        for (_, cols) in self.0.iter_mut().enumerate() {
            for (_, cell) in cols.0.iter_mut().enumerate() {
                
                cell.clear();
            }
        }
    }

    pub fn len(&self) -> u16 {
        
        self.0.len() as u16
    }
}
