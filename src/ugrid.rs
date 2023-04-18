use std::{ops::{Index, IndexMut}};
use crate::{agent::*, pool::*, cells::*};

pub const COLS: u16 = HALF_COLS * 2;
pub const ROWS: u16 = HALF_ROWS * 2;

pub const CELL_SIZE: f32 = 100.0;
pub const CELL_RADIUS: f32 = 50.0;

const HALF_COLS: u16 = 10;
const HALF_ROWS: u16 = 6;
const COL_START: f32 = CELL_SIZE * HALF_COLS as f32;
const ROW_START: f32 = CELL_SIZE * HALF_ROWS as f32;
const GRID_WIDTH: f32 = COLS as f32 * CELL_SIZE;
const GRID_HEIGHT: f32 = ROWS as f32 * CELL_SIZE;

const INV_CELL_SIZE: f32 = 1.0 / CELL_SIZE;


#[derive(Debug)]
pub struct UGrid{
    pub cells: Rows,
    pub pool: Pool,
    agent_radius: f32,
    check_radius: f32,
    check_radius_i16: i16,
}


impl Default for UGrid {

    fn default() -> Self {
        
        Self{
            cells: Rows::default(),
            pool: Pool::default(),
            agent_radius: 0.0,
            check_radius: 0.0,
            check_radius_i16: 0,
        }
    }
}

impl Index<(u16, u16)> for UGrid {
    type Output = Agent;
    
    fn index(&self, index: (u16, u16)) -> &Self::Output {
        let (i, j) = index;
        let head = self.cells[i][j].head;
        &self.pool[head]
    }
}

impl IndexMut<(u16, u16)> for UGrid {

    fn index_mut(&mut self, index: (u16, u16)) -> &mut Self::Output {
        let (i, j) = index;
        let head = self.cells[i][j].head;
        &mut self.pool[head]
    }
}



impl UGrid {
    

    pub fn new(radius: f32) -> Self {
        
        Self {
            agent_radius: radius,
            check_radius: radius * 2.0,
            check_radius_i16: (radius * 2.0) as i16,
            ..Default::default()
        }
    }

    pub fn agent_radius(&self) -> f32 {
        self.agent_radius
    }

    pub fn insert(&mut self, id: u32, x:f32, y:f32) {

        assert!(id != INACTIVE);

        let (col, row) = pos2cell(x, y);

        let mut agent = Agent::new(id, x as i16, y as i16);

        if self.cells[row][col].head != INVALID {

            agent.next = self.cells[row][col].head;
        }

        let index = self.pool.insert(agent);

        self.cells[row][col].head = index;

    }


    pub fn remove(&mut self, id: u32, x:f32, y:f32) {

        assert!(id != INACTIVE);

        let (col, row) = pos2cell(x, y);

        let index = self.pop_cell(id, row, col);

        self.pool.erase(index);
    }


    pub fn move_cell(&mut self, id: u32, prev_x: f32, prev_y: f32, x: f32, y: f32) {

        assert!(id != INACTIVE);

        let (prev_col, prev_row) = pos2cell(prev_x, prev_y);
        let (col, row) = pos2cell(x, y);

        // println!("({},{}) -> ({},{})", prev_col, prev_row, col, row);

        let index: u16;

        if prev_col == col && prev_row == row {

            index = self.find_in_cell(id, row, col);
        }
        else {

            index = self.pop_cell(id, prev_row, prev_col);

            self.push_cell(index, row, col);
        }

        if index == INVALID {
            panic!("index:{} id:{} prev:({},{}) curr:({},{}) ", index, id, prev_col, prev_row, col, row);
        }

        self.pool[index].x = x as i16;
        self.pool[index].y = y as i16;

    }


    pub fn query(&self, x: f32, y: f32, omit_id: u32) -> Vec<u16> {
        let (min_col, min_row) = pos2cell(x - self.check_radius, y + self.check_radius);
        let (max_col, max_row) = pos2cell(x + self.check_radius, y - self.check_radius);

        let mut vec: Vec<u16> = Vec::new();
        let mut index: u16;
        for row in min_row..=max_row {
            for col in min_col..=max_col {

                index = self.cells[row][col].head;

                while index != INVALID {
                    let agent = self.pool[index];

                    if (agent.id != omit_id) &&
                        agent.in_grid() &&
                        agent.is_bump_xy(x as i16, y as i16, self.check_radius_i16) {

                        vec.push(index);
                    }

                    index = agent.next;
                }
            }
        }

        vec
    }

    pub fn dir_query(&self, dir: u8, x: f32, y: f32, omit_id: u32) -> Vec<u16> {
        let (min_col, min_row) = pos2cell(x - self.check_radius, y + self.check_radius);
        let (max_col, max_row) = pos2cell(x + self.check_radius, y - self.check_radius);

        let mut vec: Vec<u16> = Vec::new();
        let mut index: u16;
        for row in min_row..=max_row {
            for col in min_col..=max_col {

                index = self.cells[row][col].head;

                while index != INVALID {
                    let agent = self.pool[index];

                    if agent.id != omit_id &&
                        agent.in_grid() &&
                        agent.bump_front_xy(dir, x as i16, y as i16, self.check_radius_i16) {

                        vec.push(index);
                    }

                    index = agent.next;
                }
            }
        }

        vec
    }

    pub fn find_in_pool(&mut self, id: u32) -> u16 {

        assert!(id != INACTIVE);

        self.pool.find(id)
    }

    pub fn find_in_cell(&mut self, id: u32, row: u16, col: u16) -> u16 {

        assert!(row != INVALID);
        assert!(col != INVALID);

        let mut index = self.cells[row][col].head;

        loop {

            if index == INVALID {
                panic!("id:{} cell:({},{}) index:{}", id, col, row, index);
            }

            if self.pool[index].id == id {
                return index;
            }

            index = self.pool[index].next;
        }
    }


    pub fn in_cell(&mut self, id: u32, row: u16, col: u16) -> bool {

        let mut index = self.cells[row][col].head;

        loop {

            if index == INVALID {
                return false;
            }

            if self.pool[index].id == id {
                return true;
            }

            index = self.pool[index].next;
        }
    }


    pub fn pop_cell(&mut self, id: u32, row: u16, col: u16) -> u16 {

        let mut index = self.cells[row][col].head;

        let mut prev = index;
        loop {
            if index == INVALID {
                panic!("id:{} cell:({},{}) index:{}", id, row, col, index);
            }

            if self.pool[index].id == id {
                break;
            }

            prev = index;
            index = self.pool[index].next;
        }

        if index == self.cells[row][col].head {
            self.cells[row][col].head = self.pool[index].next;
        }
        else {
            self.pool[prev].next = self.pool[index].next;
        }

        index
    }


    pub fn push_cell(&mut self, index: u16, row: u16, col: u16) {

        if index == INVALID {
            panic!("cell:({},{}) index:{}", row, col, index);
        }

        let head = self.cells[row][col].head;
        self.cells[row][col].head = index;
        
        self.pool[index].next = head;
    }


    pub fn print_agents(&self, row: u16, col: u16) {

        let mut index = self.cells[row][col].head;

        while index != INVALID {
            let agent = self.pool[index];

            let prev = index;
            index = agent.next;

            if !agent.is_free() {
                print!("{:4}: ", prev);
                agent.print();
            }
        }

    }


    pub fn print_cells(&self) {

        for i in 0..ROWS {
            for j in 0..COLS {
                print!("{:5} ", self.cells[i][j].head)
            }
            println!()
        }
    }

    pub fn print_pool(&self) {
        self.pool.print();
    }

    pub fn print_query(&self, indices:&Vec<u16>) {
        for index in indices {
            print!("{:2}: ", index);
            self.pool[*index].print();
        }
    }

    pub fn print_dir_query(&self, dir: u8, indices:&Vec<u16>) {
        for index in indices {
            print!("{} {:2}: ", dir, index);
            self.pool[*index].print();
        }
    }

    pub fn init_test_data(&mut self) {
        self.insert(100, 54.3, 29.4);
        self.insert(101, 12.3, 23.4);
        self.insert(102, -123.3, 223.4);
        self.insert(103, -323.3, -123.4);
        self.insert(104, 123.3, -123.4);
        self.insert(105, 423.3, 223.4);

        self.insert(106, 24.5, 62.3);
        self.insert(107, 35.5, 35.3);
        self.insert(108, 42.5, 43.3);
        self.insert(109, 21.5, 23.3);
    }
}

pub fn in_grid(x: f32, y: f32) -> bool {
    let (dx, dy) = pos2grid(x, y);

    dx >= 0.0 && dx < GRID_WIDTH &&
    dy >= 0.0 && dy < GRID_HEIGHT
}

pub fn pos2grid(x:f32, y:f32) -> (f32, f32) {
    (COL_START + x, ROW_START - y)
}

pub fn grid2pos(x:f32, y:f32) -> (f32, f32) {
    (x - COL_START, ROW_START - y)
}

pub fn cell2pos(row: u16, col: u16) -> (f32, f32) {
    let dx = (col as f32) / INV_CELL_SIZE;
    let dy = (row as f32) / INV_CELL_SIZE;
    
    (dx - COL_START, ROW_START - dy)
}

pub fn pos2cell(x:f32, y:f32) -> (u16, u16) {

    let (dx, dy) = pos2grid(x, y);

    let col = dx * INV_CELL_SIZE;
    let row = dy * INV_CELL_SIZE;

    (u16clamp(col, COLS), u16clamp(row, ROWS))
}

pub fn u16clamp(x:f32, max:u16) -> u16 {
    return x.clamp(0.0, (max - 1) as f32) as u16;
}
