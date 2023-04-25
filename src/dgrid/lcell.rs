use std::cmp::{min, max};
use crate::*;
use super::rect::*;


#[derive(Debug, Clone)]
pub struct LCell {
    pub head: u16,
    pub rect: LRect,
}


impl Default for LCell {

    fn default() -> Self {
        
        Self {
            head: INVALID,
            rect: LRect::default(),
        }
    }
}

impl LCell {

    pub fn new(head:u16) -> Self {

        Self {
            head,
            ..Default::default()
        }
    }

    pub fn expand(&mut self, x:i16, y:i16, hw:i16, hh:i16) {

        self.rect.l = min(self.rect.l, x - hw);
        self.rect.r = max(self.rect.r, x + hw);
        self.rect.b = min(self.rect.b, y - hh);
        self.rect.t = max(self.rect.t, y + hh);
    }
}