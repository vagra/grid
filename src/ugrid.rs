#![allow(dead_code)]

use std::{ops::{Index, IndexMut}};
use crate::{agent::*, pool::*, cells::*};


#[derive(Debug)]
pub struct UGrid{
    pub cells: Rows,
    pub pool: Pool,

    pub cols: u16,
    pub rows: u16,
    pub cell_size: f32,
    cell_radius: f32,
    half_cols: u16,
    half_rows: u16,
    col_max: i16,
    row_max: i16,

    half_grid_width: f32,
    half_grid_height: f32,
    grid_width: f32,
    grid_height: f32,

    inv_cell_size: f32,

    agent_radius: f32,
    pub agent_size: f32,
    agent_size_i16: i16,
}


impl Default for UGrid {

    fn default() -> Self {
        
        Self{
            cells: Rows::new(12, 20),
            pool: Pool::default(),
            
            cols: 20,
            rows: 12,
            cell_size: 100.0,
            cell_radius: 50.0,
            half_cols: 10,
            half_rows: 6,
            col_max: 19,
            row_max: 11,

            half_grid_width: 1000.0,
            half_grid_height: 600.0,
            grid_width: 2000.0,
            grid_height: 1200.0,

            inv_cell_size: 0.01,

            agent_radius: 10.0,
            agent_size: 20.0,
            agent_size_i16: 20,
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
            agent_radius:f32, cell_radius:f32,
            half_cols:u16, half_rows:u16,
        ) -> Self {
        
        Self {
            cell_radius: cell_radius,
            cell_size: cell_radius * 2.0,

            half_cols: half_cols,
            half_rows: half_rows,
            cols: half_cols * 2,
            rows: half_rows * 2,
            col_max: (half_cols * 2 - 1) as i16,
            row_max: (half_rows * 2 - 1) as i16,

            half_grid_width: half_cols as f32 * cell_radius * 2.0,
            half_grid_height: half_rows as f32 * cell_radius * 2.0,
            grid_width: half_cols as f32 * cell_radius * 4.0,
            grid_height: half_rows as f32 * cell_radius * 4.0,

            agent_radius: agent_radius,
            agent_size: agent_radius * 2.0,
            agent_size_i16: (agent_radius * 2.0) as i16,

            inv_cell_size: 0.5 / cell_radius,
            
            cells: Rows::new(half_rows * 2, half_cols * 2),
            pool: Pool::default(),
        }
    }


    pub fn agent_radius(&self) -> f32 {
        self.agent_radius
    }


    pub fn insert(&mut self, id: u32, x:f32, y:f32) {

        assert!(id != INACTIVE);

        let (col, row) = self.pos2cell(x, y);

        let mut agent = Agent::new(id, x as i16, y as i16);

        if self.cells[row][col].head != INVALID {

            agent.next = self.cells[row][col].head;
        }

        let index = self.pool.insert(agent);

        self.cells[row][col].head = index;

    }


    pub fn remove(&mut self, id: u32, x:f32, y:f32) {

        assert!(id != INACTIVE);

        let (col, row) = self.pos2cell(x, y);

        let index = self.pop_cell(id, row, col);

        self.pool.erase(index);
    }


    pub fn move_cell(&mut self, id: u32, prev_x: f32, prev_y: f32, x: f32, y: f32) {

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

        self.pool[index].x = x as i16;
        self.pool[index].y = y as i16;

    }


    pub fn query(&self, x: f32, y: f32, omit_id: u32) -> Vec<u16> {
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
                        agent.is_bump_xy(x as i16, y as i16, self.agent_size_i16) {

                        vec.push(index);
                    }

                    index = agent.next;
                }
            }
        }

        vec
    }

    pub fn dir_query(&self, dir: u8, x: f32, y: f32, omit_id: u32) -> Vec<u16> {
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
                        agent.bump_front_xy(dir, x as i16, y as i16, self.agent_size_i16) {

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
        
    pub fn in_grid(&self, x: f32, y: f32) -> bool {
        let (dx, dy) = self.pos2grid(x, y);

        dx >= 0.0 && dx < self.grid_width &&
        dy >= 0.0 && dy < self.grid_height
    }

    pub fn pos2grid(&self, x:f32, y:f32) -> (f32, f32) {
        (self.half_grid_width + x, self.half_grid_height - y)
    }

    pub fn grid2pos(&self, x:f32, y:f32) -> (f32, f32) {
        (x - self.half_grid_width, self.half_grid_height - y)
    }

    pub fn cell2pos(&self, col: u16, row: u16) -> (f32, f32) {
        let dx = (col as f32) / self.inv_cell_size;
        let dy = (row as f32) / self.inv_cell_size;
        
        self.grid2pos(dx, dy)
    }

    pub fn pos2cell(&self, x:f32, y:f32) -> (u16, u16) {

        let (dx, dy) = self.pos2grid(x, y);

        let col = dx * self.inv_cell_size;
        let row = dy * self.inv_cell_size;

        ((col as i16).clamp(0, self.col_max) as u16,
         (row as i16).clamp(0, self.row_max) as u16)
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

        for i in 0..self.rows {
            for j in 0..self.cols {
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
