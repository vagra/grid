use std::{ops::{Index, IndexMut}};
use crate::*;
use super::*;




#[derive(Debug, Clone, Copy, PartialEq)]
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


impl Agent {

    pub fn new(id: u32, x: i16, y: i16) -> Self {

        Self {
            id: id,
            x: x,
            y: y,
            
            ..Default::default()
        }
    }

    pub fn disable(&mut self) {

        self.id = INACTIVE;
    }

    pub fn is_free(&self) -> bool {

        self.id == INACTIVE
    }

    pub fn in_grid(&self, grid:&DGrid) -> bool {
        grid.in_grid(self.x, self.y)
    }

    pub fn is_bump(&self, other:&Agent, check_radius:i16) -> bool {

        self.is_bump_xy(other.x, other.y, check_radius)
    }

    pub fn is_bump_xy(&self, x:i16, y:i16, check_radius:i16) -> bool {

        (self.x - x).abs() <= check_radius && 
        (self.y - y).abs() <= check_radius
    }

    pub fn is_bump_dxy(&self, dx:i16, dy:i16, check_radius:i16) -> bool {

        dx.abs() <= check_radius && 
        dy.abs() <= check_radius
    }

    pub fn bump_front(&self, other:&Agent, dir:u8, check_radius:i16) -> bool {

        self.bump_front_xy(dir, other.x, other.y, check_radius)
    }

    pub fn bump_front_xy(&self, dir:u8, x:i16, y:i16, check_radius:i16) -> bool {
        let dx = self.x - x;
        let dy = self.y - y;
        
        self.is_bump_dxy(dx, dy, check_radius) &&
        Self::at_front_dxy(dir, dx, dy)
    }

    pub fn at_front(&self, other:&Agent, dir:u8) -> bool {

        self.at_front_xy(dir, other.x, other.y)
    }

    fn at_front_xy(&self, dir:u8, x:i16, y:i16) -> bool {

        let dx = self.x - x;
        let dy = self.y - y;
        
        Self::at_front_dxy(dir, dx, dy)
    }

    fn at_front_dxy(dir:u8, dx:i16, dy:i16) -> bool {

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

    pub fn print(&self) {
        println!("{{id:{:3}, x:{:3}, y:{:3}, next:{:5}, next_free:{:5}}}", 
            self.id, self.x, self.y, self.next, self.next_free
        );
    }

}



#[derive(Debug)]
pub struct Agents(pub Vec<Agent>);


impl Default for Agents {
    fn default() -> Self {
        
        Self(Vec::new())
    }
}

impl Index<u16> for Agents {
    type Output = Agent;

    fn index(&self, index: u16) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<u16> for Agents {

    fn index_mut(&mut self, index: u16) -> &mut Self::Output {

        &mut self.0[index as usize]
    }
}

impl Drop for Agents {

    fn drop(&mut self) {
        self.0.clear();
    }
}


impl Agents {

    pub fn find(&self, id:u32) -> u16 {
        for (i, agent) in self.0.iter().enumerate() {
            if agent.id == id {
                return i as u16;
            }
        }

        INVALID
    }

    pub fn print(&self) {
        for (i, agent) in self.0.iter().enumerate() {
            print!("{:3}: ", i);
            agent.print();
        }

        println!();
    }
}