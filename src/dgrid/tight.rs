use grid_derive::GridComm;
use std::ops::{Index, IndexMut};
use super::{*, titem::*, tcell::*, rect::*};


#[derive(Debug, GridComm)]
pub struct Tight {
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

    pub cells: Cells<TCell>,
    pub pool: Pool<TItem>,
}


impl Default for Tight {

    fn default() -> Self {
        
        Self{
            cols: 16,
            rows: 9,
            cell_size: 120,
            col_max: 15,
            row_max: 8,

            inv_cell_size: 1.0 / 120.0,

            half_width: 960,
            half_height: 540,
            width: 1920,
            height: 1080,

            cells: Cells::new(9, 16),
            pool: Pool::default(),
        }
    }
}

impl Index<(u16, u16)> for Tight {
    type Output = TItem;
    
    fn index(&self, index: (u16, u16)) -> &Self::Output {
        let (i, j) = index;
        let lhead = self.cells[i][j].lhead;
        &self.pool[lhead]
    }
}

impl IndexMut<(u16, u16)> for Tight {

    fn index_mut(&mut self, index: (u16, u16)) -> &mut Self::Output {
        let (i, j) = index;
        let lhead = self.cells[i][j].lhead;
        &mut self.pool[lhead]
    }
}



impl Tight {

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

    pub fn insert(&mut self, lcol:u16, lrow:u16, tcol:u16, trow:u16) {

        assert!(lcol != INVALID);
        assert!(lrow != INVALID);
        assert!(tcol != INVALID);
        assert!(trow != INVALID);

        let mut titem = TItem::new(lcol, lrow);

        if self.cells[trow][tcol].lhead != INVALID {

            titem.next = self.cells[trow][tcol].lhead;
        }

        let index = self.pool.insert(titem);

        self.cells[trow][tcol].lhead = index;
    }


    pub fn remove(&mut self, lcol:u16, lrow:u16, tcol:u16, trow:u16) {

        assert!(lcol != INVALID);
        assert!(lrow != INVALID);
        assert!(tcol != INVALID);
        assert!(trow != INVALID);

        let index = self.pop_cell(lcol, lrow, tcol, trow);

        self.pool.erase(index);
    }

    
    pub fn pop_cell(&mut self, lcol:u16, lrow:u16, tcol: u16, trow: u16) -> u16 {

        let mut index = self.cells[trow][tcol].lhead;

        let mut prev = index;
        loop {
            if index == INVALID {
                panic!("lcell:({},{}) tcell:({},{}) index:{}", lcol, lrow, tcol, trow, index);
            }

            if self.pool[index].lcol == lcol &&
                self.pool[index].lrow == lrow {
                break;
            }

            prev = index;
            index = self.pool[index].next;
        }

        if index == self.cells[trow][tcol].lhead {
            self.cells[trow][tcol].lhead = self.pool[index].next;
        }
        else {
            self.pool[prev].next = self.pool[index].next;
        }

        index
    }

    pub fn push_cell(&mut self, index: u16, tcol: u16, trow: u16) {

        if index == INVALID {
            panic!("tcell:({},{}) index:{}", tcol, trow, index);
        }

        let head = self.cells[trow][tcol].lhead;
        self.cells[trow][tcol].lhead = index;
        
        self.pool[index].next = head;
    }

    pub fn box2trect(&self, x:i16, y:i16, hw:i16, hh:i16) -> TRect {

        let (gx, gy) = self.pos2grid(x, y);

        self.grect2trect(gx - hw, gy - hh, gx + hw, gy + hh)
    }

    pub fn lrect2trect(&self, rect:&LRect) -> TRect {

        let (gl, gt) = self.pos2grid(rect.l, rect.t);
        let (gr, gb) = self.pos2grid(rect.r, rect.b);
        
        self.grect2trect(gl, gt, gr, gb)
    }

    pub fn grect2trect(&self, l:i16, t:i16, r:i16, b:i16) -> TRect {

        let min_col = ((l as f32) * self.inv_cell_size) as u16;
        let max_col = ((r as f32) * self.inv_cell_size) as u16;
        let min_row = ((t as f32) * self.inv_cell_size) as u16;
        let max_row = ((b as f32) * self.inv_cell_size) as u16;

        TRect {
            l: min_col.clamp(0, self.col_max),
            t: min_row.clamp(0, self.row_max),
            r: max_col.clamp(0, self.col_max),
            b: max_row.clamp(0, self.row_max),
        }
    }

    pub fn print_cells(&self) {

        println!("grid.tight: width:{} height:{}",
            self.width, self.height
        );
        println!("grid.tight.cells: cols:{} rows:{} cell_size:{}",
            self.cols, self.rows, self.cell_size);

        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.cells[i][j].lhead == INVALID {
                    print!("[ ]") 
                }
                else {
                    print!("{:2} ", self.cells[i][j].lhead)
                }
            }
            println!()
        }
    }

    pub fn print_agents(&self, grid:&DGrid) {
        for row in 0..self.rows {
            for col in 0..self.cols {

                self.print_cell_agents(grid, row, col);
            }
        }  
    }

    pub fn print_cell_agents(&self, grid:&DGrid, trow:u16, tcol:u16) {

        let mut lindex = self.cells[trow][tcol].lhead;

        while lindex != INVALID {

            println!("tcell:({:2},{:2}) -> lhead:{:2}", trow, tcol, lindex);

            let titem = self.pool[lindex];

            let lprev = lindex;
            lindex = titem.next;

            if !titem.is_free() {
                print!("{:5}: ", lprev);
                titem.print();

                grid.loose.print_cell_agents(titem.lrow, titem.lcol);
            }

        }
    } 

    pub fn print_pool(&self) {
        print!("grid.tight.");
        self.pool.print();
    }
}