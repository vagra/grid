use std::ops::{Index, IndexMut};
use crate::*;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TCell {
    pub lcell: u16,

    pub next: u16,
    pub next_free: u16,
}

impl Default for TCell {

    fn default() -> Self {

        Self {
            lcell: INVALID,
            next: INVALID,
            next_free: INVALID,
        }
    }
}


impl TCell {

    pub fn new(lcell: u16) -> Self {

        Self {
             lcell,
            ..Default::default()
        }
    }

    pub fn disable(&mut self) {

        self.lcell = INVALID;
    }

    pub fn is_free(&self) -> bool {

        self.lcell == INVALID
    }

    pub fn print(&self) {
        println!(
            "{{lcell:{:5}, next:{:5}, next_free:{:5}}}", 
            self.lcell, self.next, self.next_free
        );
    }

}



#[derive(Debug)]
pub struct TCells(pub Vec<TCell>);


impl Default for TCells {
    fn default() -> Self {
        
        Self(Vec::new())
    }
}

impl Index<u16> for TCells {
    type Output = TCell;

    fn index(&self, index: u16) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<u16> for TCells {

    fn index_mut(&mut self, index: u16) -> &mut Self::Output {

        &mut self.0[index as usize]
    }
}

impl Drop for TCells {

    fn drop(&mut self) {
        self.0.clear();
    }
}


impl TCells {

    pub fn find(&self, lcell:u16) -> u16 {
        for (i, cell) in self.0.iter().enumerate() {
            if cell.lcell == lcell {
                return i as u16;
            }
        }

        INVALID
    }

    pub fn print(&self) {
        for (i, cell) in self.0.iter().enumerate() {
            print!("{:3}: ", i);
            cell.print();
        }

        println!();
    }
}