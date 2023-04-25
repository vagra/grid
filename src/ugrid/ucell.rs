use crate::*;


#[derive(Debug, Clone, Copy)]
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

impl UCell {

    pub fn new(head:u16) -> Self {

        Self {
            head,
        }
    }
}
