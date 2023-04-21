use super::{*, titem::*, tcell::*};


#[derive(Debug)]
pub struct Tight {
    pub cols: u16,
    pub rows: u16,

    inv_cell_size: f32,

    pub heads: Rows<TCell>,
    pub cells: Pool<TItem>,
}
