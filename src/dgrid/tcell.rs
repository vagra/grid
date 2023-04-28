use grid_derive::CellComm;

use crate::*;


#[derive(Debug, Clone, Copy, CellComm)]
pub struct TCell {
    pub head: u16,
}


impl Default for TCell {

    fn default() -> Self {

        Self {
            head: INVALID,
        }
    }
}


impl CellSpec for TCell {

    fn clear(&mut self) {
        
        if self.head != INVALID {

            self.head = INVALID;
        }
    }
}


impl TCell {

    pub fn new(head:u16) -> Self {

        Self {
            head,
        }
    }

}
