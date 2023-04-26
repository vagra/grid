use std::ops::{Index, IndexMut};

pub mod ucell;
pub mod agent;

use crate::{
    *, cells::*, pool::*,
    ugrid::{agent::*, ucell::*}
};

use grid_derive::GridComm;


#[derive(Debug, GridComm)]
pub struct UGrid{
    pub cols: u16,
    pub rows: u16,
    pub agent_size: i16,
    pub cell_size: u16,

    inv_cell_size: f32,

    max_col: u16,
    max_row: u16,

    half_width: i16,
    half_height: i16,
    pub width: i16,
    pub height: i16,

    pub cells: Cells<UCell>,
    pub pool: Pool<Agent>,
}


impl Default for UGrid {

    fn default() -> Self {
        
        Self{
            cols: 20,
            rows: 12,
            agent_size: 20,
            cell_size: 100,

            inv_cell_size: 0.01,

            max_col: 19,
            max_row: 11,

            half_width: 1000,
            half_height: 600,
            width: 2000,
            height: 1200,

            cells: Cells::new(12, 20),
            pool: Pool::default(),
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

    pub fn new(
            agent_radius:u16, cell_radius:u16,
            half_cols:u16, half_rows:u16,
        ) -> Self {
        
        Self {
            cols: half_cols * 2,
            rows: half_rows * 2,
            agent_size: (agent_radius * 2) as i16,
            cell_size: cell_radius * 2,

            inv_cell_size: 1.0 / (cell_radius * 2) as f32,

            max_col: half_cols * 2 - 1,
            max_row: half_rows * 2 - 1,

            half_width: (half_cols * cell_radius * 2) as i16,
            half_height: (half_rows * cell_radius * 2) as i16,
            width: (half_cols * cell_radius * 2 * 2) as i16,
            height: (half_rows * cell_radius * 2 * 2) as i16,
            
            cells: Cells::new(half_rows * 2, half_cols * 2),
            pool: Pool::default(),
        }
    }

    pub fn insert(&mut self, id: u32, x:i16, y:i16) {

        assert!(id != INACTIVE);

        let (col, row) = self.pos2cell(x, y);

        let mut agent = Agent::new(id, x, y);

        if self.cells[row][col].head != INVALID {

            agent.next = self.cells[row][col].head;
        }

        let index = self.pool.insert(agent);

        self.cells[row][col].head = index;

    }


    pub fn remove(&mut self, id: u32, x:i16, y:i16) {

        assert!(id != INACTIVE);

        let (col, row) = self.pos2cell(x, y);

        let index = self.pop_cell(id, row, col);

        self.pool.erase(index);
    }


    pub fn move_cell(&mut self, id: u32, prev_x: i16, prev_y: i16, x: i16, y: i16) {

        assert!(id != INACTIVE);

        let (prev_col, prev_row) = self.pos2cell(prev_x, prev_y);
        let (col, row) = self.pos2cell(x, y);

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

        self.pool[index].x = x;
        self.pool[index].y = y;
    }


    pub fn query(&self, x: i16, y: i16, omit_id: u32) -> Vec<u16> {
        let (min_col, min_row) = self.pos2cell(x - self.agent_size, y + self.agent_size);
        let (max_col, max_row) = self.pos2cell(x + self.agent_size, y - self.agent_size);

        let mut vec: Vec<u16> = Vec::new();
        let mut index: u16;
        for row in min_row..=max_row {
            for col in min_col..=max_col {

                index = self.cells[row][col].head;

                while index != INVALID {
                    let agent = self.pool[index];

                    if (agent.id != omit_id) &&
                        agent.in_grid(self) &&
                        agent.is_bump_xy(x, y, self.agent_size) {

                        vec.push(index);
                    }

                    index = agent.next;
                }
            }
        }

        vec
    }

    pub fn dir_query(&self, dir: u8, x: i16, y: i16, omit_id: u32) -> Vec<u16> {
        let (min_col, min_row) = self.pos2cell(x - self.agent_size, y + self.agent_size);
        let (max_col, max_row) = self.pos2cell(x + self.agent_size, y - self.agent_size);

        let mut vec: Vec<u16> = Vec::new();
        let mut index: u16;
        for row in min_row..=max_row {
            for col in min_col..=max_col {

                index = self.cells[row][col].head;

                while index != INVALID {
                    let agent = self.pool[index];

                    if agent.id != omit_id &&
                        agent.in_grid(self) &&
                        agent.bump_front_xy(dir, x, y, self.agent_size) {

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

    pub fn pos2cell(&self, x:i16, y:i16) -> (u16, u16) {

        let (dx, dy) = self.pos2grid(x, y);

        let col = ((dx as f32) * self.inv_cell_size) as u16;
        let row = ((dy as f32) * self.inv_cell_size) as u16;

        (col.clamp(0, self.max_col),
        row.clamp(0, self.max_row))
    }

    pub fn print_agents(&self) {

        for row in 0..self.rows {
            for col in 0..self.cols {

                self.print_cell_agents(row, col);
            }
        }

    }

    pub fn print_cell_agents(&self, row: u16, col: u16) {

        let mut index = self.cells[row][col].head;

        while index != INVALID {

            println!("cell:({:2},{:2}) -> head:{:2}", row, col, index);

            let agent = self.pool[index];

            let prev = index;
            index = agent.next;

            if !agent.is_free() {
                print!("{:5}: ", prev);
                agent.print();
            }
        }
    }

    pub fn print_cells(&self) {

        println!("grid: width:{} height:{}",
            self.width, self.height
        );
        println!("grid.cells: cols:{} rows:{} cell_size:{}",
            self.cols, self.rows, self.cell_size);

        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.cells[i][j].head == INVALID {
                    print!("[ ]")
                }
                else {
                    print!("{:2} ", self.cells[i][j].head)
                }
            }
            println!()
        }
    }

    pub fn print_pool(&self) {
        print!("grid.");
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
        self.insert(100, 54, 29);
        self.insert(101, 12, 23);
        self.insert(102, -123, 223);
        self.insert(103, -323, -123);
        self.insert(104, 123, -123);
        self.insert(105, 423, 223);

        self.insert(106, 24, 62);
        self.insert(107, 35, 35);
        self.insert(108, 42, 43);
        self.insert(109, 21, 23);
    }
}
