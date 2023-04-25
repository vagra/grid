pub mod items;
pub mod cells;
pub mod pool;

pub mod ugrid;
pub mod dgrid;

pub const INACTIVE: u32 = u32::MAX;
pub const INVALID: u16 = u16::MAX;
pub const POOL_SIZE: u16 = INVALID - 1;
pub const I16MIN: i16 = i16::MIN;
pub const I16MAX: i16 = i16::MAX;
pub const U16MAX: u16 = u16::MAX;




pub trait ItemSpec {
    type ID;

    fn id(&self) -> Self::ID;
    fn set_id(&mut self, id:Self::ID);

    fn is_free(&self) -> bool;
    fn disable(&mut self);

    fn print(&self);
}

pub trait ItemComm {
    fn next(&self) -> u16;
    fn set_next(&mut self, index:u16);
    fn next_free(&self) -> u16;
    fn set_next_free(&mut self, index:u16);
}

pub trait GridComm {
    fn in_grid(&self, x: i16, y: i16) -> bool;
    fn pos2grid(&self, x:i16, y:i16) -> (i16, i16);
    fn grid2pos(&self, x:i16, y:i16) -> (i16, i16);
}
