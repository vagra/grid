pub mod agent;
pub mod pool;
pub mod tpool;


#[derive(Debug)]
pub struct DGrid{
}

impl DGrid {

    pub fn in_grid(&self, _x:i16, _y:i16) -> bool {
        true
    }
}