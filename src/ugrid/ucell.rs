use grid_derive::CellComm;
use crate::*;


#[derive(Debug, Clone, Copy, CellComm)]
pub struct UCell {
    pub head: u16,
}

impl Default for UCell {

    fn default() -> Self {

        Self {
            head: INVALID,
        }
    }
}

impl CellSpec for UCell {

    fn clear(&mut self) {

        if self.head != INVALID {

            self.head = INVALID;
        }
    }
}

impl UCell {

    pub fn new(head:u16) -> Self {

        Self {
            head,
        }
    }
}
