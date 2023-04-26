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

    pub fn pos2lcell(&self, x:i16, y:i16) -> (u16, u16) {

        let (gx, gy) = self.pos2grid(x, y);

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

    pub fn print_agents(&self, lrow:u16, lcol:u16) {

        let mut index = self.cells[lrow][lcol].head;
        
        while index != INVALID {

            println!("\tlcell:({:3},{:3}) -> head:{:5}", lcol, lrow, index);

            let agent = self.pool[index];

            let prev = index;
            index = agent.next;

            if !agent.is_free() {
                print!("\t{:5}: ", prev);
                agent.print();
            }

        }
    }

    pub fn print_pool(&self) {
        print!("grid.loose.");
        self.pool.print();
    }
}