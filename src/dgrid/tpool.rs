use std::ops::{Index, IndexMut};
use crate::*;
use super::tcell::*;


#[derive(Debug)]
pub struct TPool{ 
    pub data: TCells,
    pub size: u16,
    pub first_free: u16,
}


impl Default for TPool {
    fn default() -> Self {
        
        Self {
            data: TCells::default(),
            size: 0,
            first_free: INVALID,
        }
    }
}

impl Index<u16> for TPool {

    type Output = TCell;

    fn index(&self, index: u16) -> &Self::Output {

        &self.data[index]
    }
}

impl IndexMut<u16> for TPool {

    fn index_mut(&mut self, index: u16) -> &mut Self::Output {

        &mut self.data[index]
    }

}

impl Drop for TPool {

    fn drop(&mut self) {
        self.clear();
    }
}


impl TPool {

    pub fn insert(&mut self, tcell: TCell) -> u16 {

        if self.size >= INVALID {
            panic!("tight pool size overflow. max: {}", POOL_SIZE);
        }

        self.size += 1;

        if self.first_free != INVALID {
            let index = self.first_free;
            self.first_free = self.data[index].next_free;

            self.data[index] = tcell;

            index
        } else {
            self.data.0.push(tcell);

            self.data.0.len() as u16 - 1
        }
    }

    pub fn erase(&mut self, index: u16) {

        if index == INVALID {
            return;
        }

        if self.data.0.is_empty() {
            return;
        }

        if self.data[index].is_free() {
            return;
        }

        assert!(self.size > 0);

        self.data[index].disable();
        self.data[index].next_free = self.first_free;

        self.first_free = index;
        self.size -= 1;
    }

    pub fn clear(&mut self) {

        if self.data.0.is_empty() {
            assert_eq!(self.first_free, INVALID);
            return;
        }

        self.data.0.clear();
        self.first_free = INVALID;
        self.size = 0;
    }

    pub fn capacity(&self) -> u16 {

        self.data.0.len() as u16
    }

    pub fn find(&self, lcell:u16) -> u16 {
        self.data.find(lcell)
    }

    pub fn print(&self) {

        self.data.print();
    }
}