use crate::cells::*;
use super::lcell::*;



#[derive(Debug)]
pub struct Loose {

    pub cells: Rows<LCell>,

    pub cols: u16,
    pub rows: u16,

    inv_cell_size: f32,
}
