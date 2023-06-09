use std::cmp::{min, max};
use grid_derive::CellComm;

use crate::*;
use super::{rect::*, agent::*};


#[derive(Debug, Clone, Copy, CellComm)]
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

impl CellSpec for LCell {

    fn clear(&mut self) {
        
        if self.head != INVALID {

            self.head = INVALID;

            self.rect.l = I16MAX;
            self.rect.t = I16MIN;
            self.rect.r = I16MIN;
            self.rect.b = I16MAX;
        }
    }
}

impl LCell {

    pub fn new(head:u16) -> Self {

        Self {
            head,
            rect: LRect::default(),
        }
    }

    pub fn reset_rect(&mut self) {

        self.rect.reset();
    }

    pub fn expand_agent(&mut self, agent: &Agent) {

        self.expand(agent.x, agent.y, agent.hw, agent.hh);
    }

    pub fn expand(&mut self, x:i16, y:i16, hw:i16, hh:i16) {

        self.rect.l = min(self.rect.l, x - hw);
        self.rect.r = max(self.rect.r, x + hw);
        self.rect.b = min(self.rect.b, y - hh);
        self.rect.t = max(self.rect.t, y + hh);
    }
}