use std::{ops::{Index, IndexMut}};


#[derive(Debug, Clone)]
pub struct Cols<T>(Vec<T>);


impl<T:Default> Default for Cols<T> {

    fn default() -> Self {
        
        Self(Vec::default())
    }
}

impl<T> Index<u16> for Cols<T> {
    type Output = T;

    fn index(&self, index: u16) -> &Self::Output {
        
        &self.0[index as usize]
    }
}

impl<T> IndexMut<u16> for Cols<T> {

    fn index_mut(&mut self, index: u16) -> &mut Self::Output {

        &mut self.0[index as usize]
    }
}


impl<T:Default> Cols<T> {

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
pub struct Cells<T>(Vec<Cols<T>>);


impl<T:Default> Default for Cells<T> {

    fn default() -> Self {
        
        Self(Vec::default())
    }
}

impl<T> Index<u16> for Cells<T> {
    type Output = Cols<T>;

    fn index(&self, index: u16) -> &Self::Output {
        
        &self.0[index as usize]
    }
}

impl<T> IndexMut<u16> for Cells<T> {

    fn index_mut(&mut self, index: u16) -> &mut Self::Output {

        &mut self.0[index as usize]
    }
}

impl<T:Default> Cells<T> {

    pub fn new(rows: u16, cols: u16) -> Self {

        let mut vec: Vec<Cols<T>> = Vec::new();

        for _ in 0..rows {

            vec.push(Cols::new(cols));
        }

        Self(vec)
    }

    pub fn len(&self) -> u16 {
        
        self.0.len() as u16
    }
}
