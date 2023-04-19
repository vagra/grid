use crate::*;
use super::{*, tpool::*, cells::*};



#[derive(Debug)]
pub struct Tight {
    pub cols: u16,
    pub rows: u16,

    inv_cell_size: f32,

    pub cells: TPool,
    pub heads: Rows,
}
