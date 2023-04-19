pub mod cells;

pub mod ugrid;
pub mod dgrid;

pub const INACTIVE: u32 = u32::MAX;
pub const INVALID: u16 = u16::MAX;
pub const POOL_SIZE: u16 = INVALID - 1;