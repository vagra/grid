use grid_derive::ItemComm;
use crate::*;


#[derive(Debug, Clone, Copy, PartialEq, ItemComm)]
pub struct TItem {
    pub lcol: u16,
    pub lrow: u16,

    pub next: u16,
    pub next_free: u16,
}

impl Default for TItem {

    fn default() -> Self {

        Self {
            lcol: INVALID,
            lrow: INVALID,
            next: INVALID,
            next_free: INVALID,
        }
    }
}


impl ItemSpec for TItem {
    type ID = u16;

    fn id(&self) -> Self::ID {
        self.lcol
    }

    fn set_id(&mut self, id:Self::ID) {
        self.lcol = id
    }

    fn is_free(&self) -> bool {

        self.lcol == INVALID
    }

    fn disable(&mut self) {

        self.lcol = INVALID;
    }

    fn print(&self) {
        println!("{{lcell:({:3},{:3}), next:{:5}, next_free:{:5}}}", 
            self.lcol, self.lrow, self.next, self.next_free
        );
    }
}



impl TItem {

    pub fn new(lcol: u16, lrow: u16) -> Self {

        Self {
             lcol,
             lrow,
            ..Default::default()
        }
    }
}