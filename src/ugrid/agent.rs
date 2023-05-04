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


    pub fn cross(&self, other:&Agent, agent_size:i16) -> bool {

        self.cross_pos(other.x, other.y, agent_size)
    }

    pub fn cross_pos(&self, x:i16, y:i16, agent_size:i16) -> bool {

        self.cross_dpos(self.x - x, self.y - y, agent_size)
    }

    pub fn cross_dpos(&self, dx:i16, dy:i16, agent_size:i16) -> bool {

        dx.abs() <= agent_size && 
        dy.abs() <= agent_size
    }


    pub fn at_front(&self, dir:u8, other:&Agent) -> bool {

        self.pos_at_front(dir, other.x, other.y)
    }

    fn pos_at_front(&self, dir:u8, x:i16, y:i16) -> bool {

        self.dpos_at_front(dir, self.x - x, self.y - y)
    }

    fn dpos_at_front(&self, dir:u8, dx:i16, dy:i16) -> bool {
        
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

    pub fn cross_dirs(&self, dirs:&mut [bool;8], other:&Agent) {

        self.dpos_cross_dirs(dirs, self.x - other.x, self.y - other.y)
    }

    pub fn pos_cross_dirs(&self, dirs:&mut [bool;8], x:i16, y:i16) {

        self.dpos_cross_dirs(dirs, self.x - x, self.y - y)
    }

    pub fn dpos_cross_dirs(&self, dirs:&mut [bool;8], dx:i16, dy:i16) {

        if dx >= 0 {

            if dy >= dx.abs() {

                dirs[2] = true;
                dirs[3] = true;
                dirs[4] = true;
                dirs[5] = true;
                return;                
            }

            if dy >= 0 {
                dirs[1] = true;
                dirs[2] = true;
                dirs[3] = true;
                dirs[4] = true;
                return;
            }

            if dy < -dx.abs() {
                dirs[7] = true;
                dirs[0] = true;
                dirs[1] = true;
                dirs[2] = true;
                return;
            }
            
            dirs[0] = true;
            dirs[1] = true;
            dirs[2] = true;
            dirs[3] = true;
            return;
        }

        if dy >= dx.abs() {
            dirs[3] = true;
            dirs[4] = true;
            dirs[5] = true;
            dirs[6] = true;
            return;                
        }

        if dy >= 0 {
            dirs[4] = true;
            dirs[5] = true;
            dirs[6] = true;
            dirs[7] = true;
            return;
        }

        if dy < -dx.abs() {
            dirs[6] = true;
            dirs[7] = true;
            dirs[0] = true;
            dirs[1] = true;
            return;
        }
        
        dirs[5] = true;
        dirs[6] = true;
        dirs[7] = true;
        dirs[0] = true;
        return;
    }

}