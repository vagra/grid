use super::lcell::*;



#[derive(Debug)]
pub struct Loose {

    pub cells: LCells,

    pub cols: u16,
    pub rows: u16,

    inv_cell_size: f32,
}
