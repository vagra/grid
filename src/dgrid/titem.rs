use grid_derive::ItemComm;
use crate::*;


#[derive(Debug, Clone, Copy, PartialEq, ItemComm)]
pub struct TItem {
    pub lcell: u16,

    pub next: u16,
    pub next_free: u16,
}

impl Default for TItem {

    fn default() -> Self {

        Self {
            lcell: INVALID,
            next: INVALID,
            next_free: INVALID,
        }
    }
}


impl ItemSpec for TItem {
    type ID = u16;

    fn id(&self) -> Self::ID {
        self.lcell
    }

    fn set_id(&mut self, id:Self::ID) {
        self.lcell = id
    }

    fn is_free(&self) -> bool {

        self.lcell == INVALID
    }

    fn disable(&mut self) {

        self.lcell = INVALID;
    }

    fn print(&self) {
        println!("{{lcell:{:5}, next:{:5}, next_free:{:5}}}", 
            self.lcell, self.next, self.next_free
        );
    }
}



impl TItem {

    pub fn new(lcell: u16) -> Self {

        Self {
             lcell,
            ..Default::default()
        }
    }
}