use crate::*;


#[derive(Debug, Clone, Copy)]
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

impl TCell {

    pub fn new(head:u16) -> Self {

        Self {
            head: head,
        }
    }
}