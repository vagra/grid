pub mod agent;

pub mod titem;
pub mod tcell;
pub mod tight;

pub mod lcell;
pub mod loose;

use crate::{*, cells::*, pool::*};
use dgrid::{agent::*, tight::*, loose::*};

#[derive(Debug)]
pub struct DGrid{
    pub tight: Tight,
    pub loose: Loose,
    pub pool: Pool<Agent>,
}

impl DGrid {

    pub fn in_grid(&self, _x:i16, _y:i16) -> bool {
        true
    }
}