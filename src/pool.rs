use std::ops::{Index, IndexMut};
use crate::{*, items::*};


#[derive(Debug)]
pub struct Pool<T: ItemSpec+ItemComm>{ 
    pub data: Items<T>,
    pub size: u16,
    pub first_free: u16,
}


impl<T:ItemSpec+ItemComm + Default>
Default for Pool<T> {
    fn default() -> Self {
        
        Self {
            data: Items::default(),
            size: 0,
            first_free: INVALID,
        }
    }
}

impl<T:ItemSpec+ItemComm>
Index<u16> for Pool<T> {

    type Output = T;

    fn index(&self, index: u16) -> &Self::Output {

        &self.data[index]
    }
}

impl<T:ItemSpec+ItemComm>
IndexMut<u16> for Pool<T> {

    fn index_mut(&mut self, index: u16) -> &mut Self::Output {

        &mut self.data[index]
    }

}

impl<T:ItemSpec+ItemComm>
Drop for Pool<T> {

    fn drop(&mut self) {
        self.clear();
    }
}


impl<T:ItemSpec+ItemComm>
Pool<T> {

    pub fn insert(&mut self, item: T) -> u16 {

        if self.size >= INVALID {
            panic!("pool size overflow. max: {}", POOL_SIZE);
        }

        self.size += 1;

        if self.first_free != INVALID {
            let index = self.first_free;
            self.first_free = self.data[index].next_free();

            self.data[index] = item;

            index
        } else {
            self.data.0.push(item);

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
        self.data[index].set_next_free(self.first_free);

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

    pub fn find<ID>(&self, id: ID) -> u16
    where
        T::ID: PartialEq<ID>
    {
        self.data.find(id)
    }

    pub fn print(&self) {

        self.data.print();
    }
}