use crate::ItemComm;
use grid_derive::ItemComm;
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
        self.id = id
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

        /*
        println!("{},{},{},{}",
            self.l(),
            self.r(),
            self.t(),
            self.b()
        ); */

        self.l() < grid.half_width &&
        self.r() >= -grid.half_width &&
        self.t() > -grid.half_height &&
        self.b() <= grid.half_height
    }

    pub fn cross(&self, other:&Agent) -> bool {

        self.cross_box(other.x, other.y, other.hw, other.hh)
    }

    pub fn cross_box(&self, x:i16, y:i16, hw:i16, hh:i16) -> bool {

        self.l() <= x + hw &&
        self.r() >= x - hw &&
        self.t() >= y - hh &&
        self.b() <= y + hh
    }

    pub fn front_cross_box(&self, dir:u8, x:i16, y:i16, hw:i16, hh:i16) -> bool {
        
        match dir {

            1 => { self.cross_bottom(x, y, hw, hh) ||
                    self.cross_right(x, y, hw, hh) },

            2 => { self.cross_right(x, y, hw, hh) },

            3 => { self.cross_right(x, y, hw, hh) ||
                    self.cross_top(x, y, hw, hh) },

            4 => { self.cross_top(x, y, hw, hh) },

            5 => { self.cross_top(x, y, hw, hh) ||
                    self.cross_left(x, y, hw, hh) },

            6 => { self.cross_right(x, y, hw, hh) },

            7 => { self.cross_right(x, y, hw, hh) ||
                    self.cross_bottom(x, y, hw, hh) },

            _ => { self.cross_bottom(x, y, hw, hh) },
        }
    }

    fn cross_bottom(&self, x:i16, y:i16, hw:i16, hh:i16) -> bool {

        (self.t() >= y - hh) &&
        ((self.l() <= x + hw) || (self.r() >= x - hw))
    }

    fn cross_top(&self, x:i16, y:i16, hw:i16, hh:i16) -> bool {

        (self.b() <= y + hh) &&
        ((self.l() <= x + hw) || (self.r() >= x - hw))
    }

    fn cross_left(&self, x:i16, y:i16, hw:i16, hh:i16) -> bool {

        (self.r() >= x - hw) &&
        ((self.t() >= y - hh) || (self.b() <= y + hh))
    }

    fn cross_right(&self, x:i16, y:i16, hw:i16, hh:i16) -> bool {

        (self.l() <= x + hw) &&
        ((self.t() >= y - hh) || (self.b() <= y + hh)) 
    }
}