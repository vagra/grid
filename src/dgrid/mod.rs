pub mod agent;
pub mod pool;

pub mod tcell;
pub mod tpool;
pub mod tight;

pub mod lcell;
pub mod loose;

use crate::*;
use {pool::*, tight::*, loose::*};

#[derive(Debug)]
pub struct DGrid{
    pub tight: Tight,
    pub loose: Loose,
    pub pool: Pool,
}

impl DGrid {

    pub fn in_grid(&self, _x:i16, _y:i16) -> bool {
        true
    }
}