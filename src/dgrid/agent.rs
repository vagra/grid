use grid_derive::ItemComm;
use crate::*;
use super::*;


#[derive(Debug, Clone, Copy, PartialEq, ItemComm)]
pub struct Agent {
    pub id: u32,
    pub x: i16,
    pub y: i16,
    pub hw: i16,
    pub hh: i16,

    pub next: u16,
    pub next_free: u16,
}

impl Default for Agent {
    
    fn default() -> Self {

        Self {
            id: INACTIVE,
            x: 0,
            y: 0,
            hw: 0,
            hh: 0,
            next: INVALID,
            next_free: INVALID,
        }
    }
}


impl ItemSpec for Agent {
    type ID = u32;

    fn id(&self) -> Self::ID {
        self.id
    }

    fn set_id(&mut self, id:Self::ID) {
        self.id = id;
    }

    fn is_free(&self) -> bool {

        self.id == INACTIVE
    }

    fn disable(&mut self) {

        self.id = INACTIVE;
    }

    fn print(&self) {
        println!(
            "Agent{{ id:{:3}, x:{:4}, y:{:4}, hw:{:3}, hh:{:3}, next:{:5}, next_free:{:5} }}", 
            self.id, self.x, self.y, self.hw, self.hh, self.next, self.next_free
        );
    }
}


impl Agent {

    pub fn new(id:u32, x:i16, y:i16, hw:i16, hh:i16) -> Self {

        Self {
            id,
            x,
            y,
            hw,
            hh,
            
            ..Default::default()
        }
    }

    pub fn l(&self) -> i16 { self.x - self.hw }
    pub fn r(&self) -> i16 { self.x + self.hw }
    pub fn t(&self) -> i16 { self.y + self.hh }
    pub fn b(&self) -> i16 { self.y - self.hh }

    pub fn in_grid(&self, grid:&DGrid) -> bool {

        self.l() < grid.half_width &&
        self.r() >= -grid.half_width &&
        self.t() > -grid.half_height &&
        self.b() <= grid.half_height
    }

    pub fn cross(&self, other:&Agent) -> bool {

        self.l() <= other.r() &&
        self.r() >= other.l() &&
        self.t() >= other.b() &&
        self.b() <= other.t()
    }

    pub fn box_cross(&self, x:i16, y:i16, hw:i16, hh:i16) -> bool {

        self.l() <= x + hw &&
        self.r() >= x - hw &&
        self.t() >= y - hh &&
        self.b() <= y + hh
    }


    pub fn at_front(&self, dir:u8, other:&Agent) -> bool {

        self.pos_at_front(dir, other.x, other.y)
    }

    fn pos_at_front(&self, dir:u8, x:i16, y:i16) -> bool {

        dpos_at_front(dir, self.x - x, self.y - y)
    }


    pub fn cross_dirs(&self, dirs:&mut [bool;8], other:&Agent) {

        dpos_cross_dirs(dirs, self.x - other.x, self.y - other.y)
    }

    pub fn pos_cross_dirs(&self, dirs:&mut [bool;8], x:i16, y:i16) {

        dpos_cross_dirs(dirs, self.x - x, self.y - y)
    }

}
