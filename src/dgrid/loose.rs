use std::ops::{Index, IndexMut};

use grid_derive::GridComm;
use crate::{*, cells::*, pool::*};
use super::{lcell::*, agent::*};



#[derive(Debug, GridComm)]
pub struct Loose {

    pub cols: u16,
    pub rows: u16,
    pub cell_size: u16,
    col_max: u16,
    row_max: u16,

    inv_cell_size: f32,

    half_width: i16,
    half_height: i16,
    pub width: i16,
    pub height: i16,

    pub cells: Cells<LCell>,
    pub pool: Pool<Agent>,
}

impl Default for Loose {

    fn default() -> Self {
        
        Self {
            cols: 64,
            rows: 36,
            cell_size: 30,
            col_max: 63,
            row_max: 35,

            inv_cell_size: 1.0 / 30.0,

            half_width: 960,
            half_height: 540,
            width: 1920,
            height: 1080,

            cells: Cells::new(36, 64),
            pool: Pool::default(),
        }
    }
}


impl Index<(u16, u16)> for Loose {
    type Output = Agent;
    
    fn index(&self, index: (u16, u16)) -> &Self::Output {
        let (i, j) = index;
        let head = self.cells[i][j].head;
        &self.pool[head]
    }
}

impl IndexMut<(u16, u16)> for Loose {

    fn index_mut(&mut self, index: (u16, u16)) -> &mut Self::Output {
        let (i, j) = index;
        let head = self.cells[i][j].head;
        &mut self.pool[head]
    }
}


impl Loose {

    pub fn new(cols:u16, rows:u16, cell_radius:u16) -> Self {
        
        Self {
            cols,
            rows,
            cell_size: cell_radius * 2,
            col_max: cols - 1,
            row_max: rows - 1,

            inv_cell_size: 0.5 / cell_radius as f32,

            half_width: (cols * cell_radius) as i16,
            half_height: (rows * cell_radius) as i16,
            width: (cols * cell_radius * 2) as i16,
            height: (rows * cell_radius * 2) as i16,
            
            cells: Cells::new(rows, cols),
            pool: Pool::default(),
        }
    }

    pub fn clear(&mut self) {

        self.cells.clear();
        self.pool.clear();
    }

    pub fn insert(&mut self, id: u32, x:i16, y:i16, hw:i16, hh:i16) -> (u16, u16) {

        assert!(id != INACTIVE);

        let (col, row) = self.pos2lcell(x, y);

        let mut agent = Agent::new(id, x, y, hw, hh);

        if self.cells[row][col].head != INVALID {

            agent.next = self.cells[row][col].head;
        }

        let index = self.pool.insert(agent);

        self.cells[row][col].head = index;

        (col, row)
    }


    pub fn remove(&mut self, id: u32, x:i16, y:i16) {

        assert!(id != INACTIVE);

        let (col, row) = self.pos2lcell(x, y);

        let index = self.pop_cell(id, row, col);

        self.pool.erase(index);
    }

    pub fn move_lcell(&mut self, id:u32, prev_x:i16, prev_y:i16, x:i16, y:i16) {

        assert!(id != INACTIVE);

        let (prev_lcol, prev_lrow) = self.pos2lcell(prev_x, prev_y);
        let (lcol, lrow) = self.pos2lcell(x, y);

        let index: u16;

        if prev_lcol == lcol && prev_lrow == lrow {

            index = self.find_in_cell(id, lrow, lcol);
        }
        else {

            index = self.pop_cell(id, prev_lrow, prev_lcol);

            self.push_cell(index, lrow, lcol);
        }

        if index == INVALID {
            panic!("index:{} id:{} prev:({},{}) curr:({},{}) ",
                index, id, prev_lrow, prev_lcol, lrow, lcol);
        }

        self.pool[index].x = x;
        self.pool[index].y = y;

        self.expand_lrect(lcol, lrow, x, y, 
            self.pool[index].hw, self.pool[index].hh
        );
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

        let prev = self.cells[row][col].head;
        self.cells[row][col].head = index;
        
        self.pool[index].next = prev;
    }

    pub fn expand_lrect(&mut self,
        lcol:u16, lrow:u16,
        x:i16, y:i16, hw:i16, hh:i16) {

        let lcell = &mut self.cells[lrow][lcol];
        lcell.expand(x, y, hw, hh);
    }


    pub fn optimize(&mut self) {

        let mut new_pool: Pool<Agent> = Pool::default();

        let mut new_index: u16;
        let mut indices: Vec<u16>;
        let mut lcell: &mut LCell;
        let mut agent: &Agent;
        for lrow in 0..self.rows {
            for lcol in 0..self.cols {

                indices = Vec::default();

                lcell = &mut self.cells[lrow][lcol];

                while lcell.head != INVALID {

                    agent = &self.pool[lcell.head];
                    new_index = new_pool.insert(*agent);
                    indices.push(new_index);
                    lcell.head = agent.next;
                }

                for (_, new_index) in indices.iter().enumerate() {
                    new_pool[*new_index].next = lcell.head;
                    lcell.head = *new_index;
                }

            }
        }

        let _ = std::mem::replace(&mut self.pool, new_pool);
    }


    pub fn rebuild_rects(&mut self) {

        let mut lcell: &mut LCell;
        let mut index: u16;
        let mut agent: &Agent;

        for lrow in 0..self.rows {
            for lcol in 0..self.cols {

                lcell = &mut self.cells[lrow][lcol];
                lcell.reset_rect();

                index = lcell.head;

                while index != INVALID {

                    agent = &self.pool[index];
                    lcell.expand_agent(agent);

                    index = agent.next;
                }
            }
        }
    }


    pub fn find_in_cell(&mut self, id: u32, row: u16, col: u16) -> u16 {

        assert!(id != INACTIVE);
        assert!(row != INVALID);
        assert!(col != INVALID);

        let mut index = self.cells[row][col].head;

        loop {

            if index == INVALID {
                panic!("id:{} cell:({},{}) index:{}", id, row, col, index);
            }

            if self.pool[index].id == id {
                return index;
            }

            index = self.pool[index].next;
        }
    }


    pub fn lcell2pos(&self, col: u16, row: u16) -> (i16, i16) {
        let dx = (col * self.cell_size) as i16;
        let dy = (row * self.cell_size) as i16;
        
        self.grid2pos(dx, dy)
    }


    pub fn pos2lcell(&self, x:i16, y:i16) -> (u16, u16) {

        let (gx, gy) = self.pos2grid(x, y);
        // println!("({}, {}) -> ({}, {})", x, y, gx, gy);

        let col = ((gx as f32) * self.inv_cell_size) as u16;
        let row = ((gy as f32) * self.inv_cell_size) as u16;

        (col.clamp(0, self.col_max),
        row.clamp(0, self.row_max))
    }


    pub fn print_cells(&self) {

        println!("grid.loose: width:{} height:{}",
            self.width, self.height
        );
        println!("grid.loose.cells: cols:{} rows:{} cell_size:{}",
            self.cols, self.rows, self.cell_size);

        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.cells[i][j].head == INVALID {
                    print!("[]")    
                }
                else {
                    print!("{:2}", self.cells[i][j].head)
                }
            }
            println!()
        }
    }

    pub fn print_agents(&self) {

        for row in 0..self.rows {
            for col in 0..self.cols {

                self.print_cell_agents(row, col);
            }
        }
    }

    pub fn print_cell_agents(&self, lrow:u16, lcol:u16) {

        let mut head = self.cells[lrow][lcol].head;
        let rect = &self.cells[lrow][lcol].rect;
        
        if head == INVALID {
            return;
        }

        println!("  lcell:({:2},{:2}) -> head:{:2}", lrow, lcol, head);
        println!("   rect:[{:4},{:4},{:4},{:4}]", rect.l, rect.t, rect.r, rect.b);

        let mut agent: &Agent;
        let mut prev: u16;
        while head != INVALID {

            agent = &self.pool[head];

            prev = head;
            head = agent.next;

            if !agent.is_free() {
                print!("  {:5}: ", prev);
                agent.print();
            }
        }
    }

    pub fn print_pool(&self) {
        print!("grid.loose.");
        self.pool.print();
    }

    pub fn init_test_data(&mut self) {

        self.insert(101, 23, 24, 10, 10);
        self.insert(102, 12, 10, 10, 10);
        self.insert(103, 6, 23, 10, 10);
        self.insert(104, 40, 97, 10, 10);
        self.insert(105, -123, -432, 10, 10);
        self.insert(106, -234, 324, 10, 10);
        self.insert(107, 450, 123, 10, 10);
        self.insert(108, 480, 170, 10, 10);
        self.insert(109, 15, 27, 10, 10);
    }
}