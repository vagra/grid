use grid_derive::ItemComm;
use crate::*;
use super::*;


#[derive(Debug, Clone, Copy, PartialEq, ItemComm)]
pub struct Agent {
    pub id: u32,
    pub x: i16,
    pub y: i16,

    pub next: u16,
    pub next_free: u16,
}

impl Default for Agent {
    fn default() -> Self {

        Self {
            id: INACTIVE,
            x: 0,
            y: 0,
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
            "Agent{{ id:{:3}, x:{:4}, y:{:4}, next:{:5}, next_free:{:5} }}", 
            self.id, self.x, self.y, self.next, self.next_free
        );
    }
}


impl Agent {

    pub fn new(id: u32, x: i16, y: i16) -> Self {

        Self {
            id: id,
            x: x,
            y: y,
            
            ..Default::default()
        }
    }

    pub fn in_grid(&self, grid:&UGrid) -> bool {

        self.x >= -grid.half_width &&
        self.x < grid.half_width &&
        self.y > -grid.half_height &&
        self.y <= grid.half_height
    }

    pub fn cross(&self, other:&Agent, check_radius:i16) -> bool {

        self.cross_pos(other.x, other.y, check_radius)
    }

    pub fn cross_pos(&self, x:i16, y:i16, check_radius:i16) -> bool {

        (self.x - x).abs() <= check_radius && 
        (self.y - y).abs() <= check_radius
    }

    pub fn front_cross(&self, other:&Agent, dir:u8, radius:i16) -> bool {

        self.front_cross_pos(dir, other.x, other.y, radius)
    }

    pub fn front_cross_pos(&self, dir:u8, x:i16, y:i16, radius:i16) -> bool {

        match dir {

            1 => { self.cross_bottom(x, y, radius) ||
                    self.cross_right(x, y, radius) },

            2 => { self.cross_right(x, y, radius) },

            3 => { self.cross_right(x, y, radius) ||
                    self.cross_top(x, y, radius) },

            4 => { self.cross_top(x, y, radius) },

            5 => { self.cross_top(x, y, radius) ||
                    self.cross_left(x, y, radius) },

            6 => { self.cross_left(x, y, radius) },

            7 => { self.cross_left(x, y, radius) ||
                    self.cross_bottom(x, y, radius) },

            _ => { self.cross_bottom(x, y, radius) },
        }
    }

    pub fn at_front(&self, other:&Agent, dir:u8) -> bool {

        self.at_front_pos(dir, other.x, other.y)
    }

    fn at_front_pos(&self, dir:u8, x:i16, y:i16) -> bool {

        let dx = self.x - x;
        let dy = self.y - y;
        
        match dir {
            1 => dx >= 0 && dy <= 0,
            2 => dx >= dy.abs(),
            3 => dx >= 0 && dy >= 0,
            4 => dy >= dx.abs(),
            5 => dx <= 0 && dy >= 0,
            6 => dx <= -dy.abs(),
            7 => dx <= 0 && dy <= 0,
            _ => dy <= -dx.abs(),
        }
    }

    pub fn cross_bottom(&self, x:i16, y:i16, r:i16) -> bool {

        self.y + r >= y - r &&
        self.y + r <= y + r &&
        self.x - r <= x + r &&
        self.x + r >= x - r
    }

    pub fn cross_top(&self, x:i16, y:i16, r:i16) -> bool {

        self.y - r <= y + r &&
        self.y - r >= y - r &&
        self.x - r <= x + r &&
        self.x + r >= x - r
    }

    pub fn cross_left(&self, x:i16, y:i16, r:i16) -> bool {

        self.x + r >= x - r &&
        self.x + r <= x + r &&
        self.y - r <= y + r &&
        self.y + r >= y - r
    }

    pub fn cross_right(&self, x:i16, y:i16, r:i16) -> bool {

        self.x - r <= x + r &&
        self.x - r >= x - r &&
        self.y - r <= y + r &&
        self.y + r >= y - r
    }

}