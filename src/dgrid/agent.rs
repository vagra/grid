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

    pub fn in_grid(&self, grid:&DGrid) -> bool {

        /*
        println!("{},{},{},{}",
            self.x - self.hw,
            self.x + self.hw,
            self.y + self.hh,
            self.y - self.hh
        ); */

        self.x - self.hw < grid.half_width &&
        self.x + self.hw >= -grid.half_width &&
        self.y + self.hh > -grid.half_height &&
        self.y - self.hh <= grid.half_height
    }

    pub fn cross(&self, other:&Agent) -> bool {

        self.cross_box(other.x, other.y, other.hw, other.hh)
    }

    pub fn cross_box(&self, x:i16, y:i16, hw:i16, hh:i16) -> bool {

        self.x - self.hw <= x + hw &&
        self.x + self.hw >= x - hw &&
        self.y + self.hh >= y - hh &&
        self.y - self.hh <= y + hh
    }
}