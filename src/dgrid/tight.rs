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
    factor: u16,

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
            factor: 4,

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
        let lhead = self.cells[i][j].head;
        &self.pool[lhead]
    }
}

impl IndexMut<(u16, u16)> for Tight {

    fn index_mut(&mut self, index: (u16, u16)) -> &mut Self::Output {
        let (i, j) = index;
        let lhead = self.cells[i][j].head;
        &mut self.pool[lhead]
    }
}



impl Tight {

    pub fn new(cols:u16, rows:u16, cell_radius:u16, factor:u16) -> Self {
        
        Self {
            cols,
            rows,
            cell_size: cell_radius * 2,
            col_max: cols - 1,
            row_max: rows - 1,
            factor,

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

    pub fn insert(&mut self, lcol:u16, lrow:u16, tcol:u16, trow:u16) {

        assert!(lcol != INVALID);
        assert!(lrow != INVALID);
        assert!(tcol != INVALID);
        assert!(trow != INVALID);

        let mut titem = TItem::new(lcol, lrow);

        if self.cells[trow][tcol].head != INVALID {

            titem.next = self.cells[trow][tcol].head;
        }

        let index = self.pool.insert(titem);

        self.cells[trow][tcol].head = index;
    }

    pub fn insert_lcell(&mut self, lcol:u16, lrow:u16) {

        let (tcol, trow) = self.lcell2tcell(lcol, lrow);

        self.insert(lcol, lrow, tcol, trow);
    }


    pub fn remove(&mut self, lcol:u16, lrow:u16, tcol:u16, trow:u16) {

        assert!(lcol != INVALID);
        assert!(lrow != INVALID);
        assert!(tcol != INVALID);
        assert!(trow != INVALID);

        let index = self.pop_cell(lcol, lrow, tcol, trow);

        self.pool.erase(index);
    }


    pub fn remove_lcell(&mut self, lcol:u16, lrow:u16) {

        let (tcol, trow) = self.lcell2tcell(lcol, lrow);

        self.remove(lcol, lrow, tcol, trow);
    }

    
    pub fn pop_cell(&mut self, lcol:u16, lrow:u16, tcol: u16, trow: u16) -> u16 {

        let mut index = self.cells[trow][tcol].head;

        let mut prev = index;
        loop {
            if index == INVALID {
                panic!("lcell:({},{}) tcell:({},{}) index:{}",
                    lrow, lcol, trow, tcol, index);
            }

            if self.pool[index].lcol == lcol &&
                self.pool[index].lrow == lrow {
                break;
            }

            prev = index;
            index = self.pool[index].next;
        }

        if index == self.cells[trow][tcol].head {
            self.cells[trow][tcol].head = self.pool[index].next;
        }
        else {
            self.pool[prev].next = self.pool[index].next;
        }

        index
    }

    pub fn push_cell(&mut self, index: u16, tcol: u16, trow: u16) {

        if index == INVALID {
            panic!("tcell:({},{}) index:{}", trow, tcol, index);
        }

        let prev = self.cells[trow][tcol].head;
        self.cells[trow][tcol].head = index;
        
        self.pool[index].next = prev;
    }

    pub fn tcell2pos(&self, col: u16, row: u16) -> (i16, i16) {
        let dx = (col * self.cell_size) as i16;
        let dy = (row * self.cell_size) as i16;
        
        self.grid2pos(dx, dy)
    }

    pub fn lcell2tcell(&self, lcol:u16, lrow:u16) -> (u16, u16) {

        assert!(lcol < self.cols * self.factor);
        assert!(lrow < self.rows * self.factor);

        (lcol / self.factor, lrow / self.factor)
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

    pub fn print_titems(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols {

                self.print_cell_titems(row, col);
            }
        }  
    }

    pub fn print_cell_titems(&self, trow:u16, tcol:u16) {

        let mut lindex = self.cells[trow][tcol].head;

        let mut titem: &TItem;
        let mut lprev: u16;
        while lindex != INVALID {

            println!("tcell:({:2},{:2}) -> lhead:{:2}", trow, tcol, lindex);

            titem = &self.pool[lindex];

            lprev = lindex;
            lindex = titem.next;

            if !titem.is_free() {
                print!("{:5}: ", lprev);
                titem.print();
            }
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

        let mut ihead = self.cells[trow][tcol].head;

        if ihead == INVALID {
            return;
        }

        println!("tcell:({:2},{:2}) -> ihead:{:2}", trow, tcol, ihead);

        let mut titem: &TItem;
        while ihead != INVALID {

            titem = &self.pool[ihead];

            if !titem.is_free() {
                print!("{:5}: ", ihead);
                titem.print();

                grid.loose.print_cell_agents(titem.lrow, titem.lcol);
            }

            ihead = titem.next;
        }
    } 

    pub fn print_pool(&self) {
        print!("grid.tight.");
        self.pool.print();
    }

    pub fn init_test_data(&mut self) {

        self.insert_lcell(32, 17);
        self.insert_lcell(33, 15);
        self.insert_lcell(27, 32);
        self.insert_lcell(14, 7);
        self.insert_lcell(47, 13);
        self.insert_lcell(48, 12);
        self.insert_lcell(4, 6);
        self.insert_lcell(5, 7);
        self.insert_lcell(6, 8);
        self.insert_lcell(13, 24);
    }
}