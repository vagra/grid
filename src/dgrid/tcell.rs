use crate::*;


#[derive(Debug, Clone, Copy)]
pub struct TCell {
    pub lhead: u16,
}

impl Default for TCell {
    fn default() -> Self {

        Self {
            lhead: INVALID,
        }
    }
}

impl TCell {

    pub fn new(lhead:u16) -> Self {

        Self {
            lhead,
        }
    }

}
