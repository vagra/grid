use std::{ops::{Index, IndexMut}};
use crate::*;



#[derive(Debug)]
pub struct Items<T:ItemSpec+ItemComm>(pub Vec<T>);


impl<T:ItemSpec+ItemComm+Default>
Default for Items<T> {
    fn default() -> Self {
        
        Self(Vec::default())
    }
}

impl<T:ItemSpec+ItemComm>
Index<u16> for Items<T> {
    type Output = T;

    fn index(&self, index: u16) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl<T:ItemSpec+ItemComm>
IndexMut<u16> for Items<T> {

    fn index_mut(&mut self, index: u16) -> &mut Self::Output {

        &mut self.0[index as usize]
    }
}

impl<T:ItemSpec+ItemComm>
Drop for Items<T> {

    fn drop(&mut self) {
        self.0.clear();
    }
}


impl<T:ItemSpec+ItemComm>
Items<T> {

    pub fn find<ID>(&self, id:ID) -> u16
    where
        T::ID: PartialEq<ID>
    {
        for (i, item) in self.0.iter().enumerate() {
            if item.id() == id {
                return i as u16;
            }
        }

        INVALID
    }

    pub fn clear(&mut self) {

        self.0.clear();
    }

    pub fn print(&self) {
        for (i, item) in self.0.iter().enumerate() {
            print!("{:3}: ", i);
            item.print();
        }
    }
}