pub mod agent;
pub mod rect;

pub mod titem;
pub mod tcell;
pub mod tight;

pub mod lcell;
pub mod loose;

use crate::{
    *, cells::*, pool::*,
    dgrid::{tight::*, loose::*}
};
use grid_derive::GridComm;

#[derive(Debug, GridComm)]
pub struct DGrid{

    half_width: i16,
    half_height: i16,
    pub width: i16,
    pub height: i16,

    pub tight: Tight,
    pub loose: Loose,
}


impl Default for DGrid {

    fn default() -> Self {
        
        Self{

            half_width: 960,
            half_height: 540,
            width: 1920,
            height: 1080,

            tight: Tight::default(),
            loose: Loose::default(),
        }
    }
}


impl DGrid {

    pub fn new(factor:u16, lcell_radius:u16,
        half_tcols:u16, half_trows:u16,
    ) -> Self {

        Self {
            half_width: (half_tcols * lcell_radius * factor * 2) as i16,
            half_height: (half_trows * lcell_radius * factor * 2) as i16,
            width: (half_tcols * lcell_radius * factor * 2 * 2) as i16,
            height: (half_trows * lcell_radius * factor * 2 * 2) as i16,
            
            tight: Tight::new(
                half_tcols * 2, half_trows * 2, lcell_radius * factor, factor),
            
            loose: Loose::new(
                half_tcols * factor * 2, half_trows * factor * 2, lcell_radius),
        }
    }


    pub fn insert(&mut self, id: u32, x:i16, y:i16, hw:i16, hh:i16) {

        let (lcol, lrow) = self.loose.insert(id, x, y, hw, hh);

        self.expand_aabb(lcol, lrow, x, y, hw, hh);
    }
    
    pub fn remove(&mut self, id:u32, x:i16, y:i16) {

        self.loose.remove(id, x, y);
    }

    pub fn move_cell(&mut self, id:u32, prev_x:i16, prev_y:i16, x:i16, y:i16) {

        assert!(id != INACTIVE);

        let (prev_lcol, prev_lrow) = self.loose.pos2lcell(prev_x, prev_y);
        let (lcol, lrow) = self.loose.pos2lcell(x, y);

        let index: u16;

        print!("id:{:3}, ({:2},{:2}) -> ({:2},{:2}), ",
            id, prev_x, prev_y, x, y
        );
        println!("({:2},{:2}) -> ({:2},{:2})",
            prev_lrow, prev_lcol, lrow, lcol
        );

        if prev_lcol == lcol && prev_lrow == lrow {

            index = self.loose.find_in_cell(id, lrow, lcol);
        }
        else {

            index = self.loose.pop_cell(id, prev_lrow, prev_lcol);

            self.loose.push_cell(index, lrow, lcol);
        }

        if index == INVALID {
            panic!("index:{} id:{} prev:({},{}) curr:({},{}) ",
                index, id, prev_lrow, prev_lcol, lrow, lcol);
        }

        self.loose.pool[index].x = x;
        self.loose.pool[index].y = y;

        self.expand_aabb(lcol, lrow, x, y, 
            self.loose.pool[index].hw, self.loose.pool[index].hh
        );
    }


    pub fn optimize(&mut self) {
        
        self.tight.clear();
        self.loose.optimize();
    }


    pub fn expand_aabb(&mut self, lcol:u16, lrow:u16,
        x:i16, y:i16, hw:i16, hh:i16) {

        let lcell = &mut self.loose.cells[lrow][lcol];
        let prev_lrect = lcell.rect.clone();
        lcell.expand(x, y, hw, hh);

        let trect = self.tight.box2trect(x, y, hw, hh);
        // println!("trect: [{},{},{},{}]", trect.l, trect.t, trect.r, trect.b);

        if prev_lrect.is_empty() {

            for trow in trect.t..=trect.b {
                for tcol in trect.l..=trect.r {
                    self.tight.insert(lcol, lrow, tcol, trow);
                }
            }

            return;
        }

        let prev_trect = self.tight.lrect2trect(&prev_lrect);

        if trect.l != prev_trect.l || trect.r != prev_trect.r ||
            trect.t != prev_trect.t || trect.b != prev_trect.b {

            for trow in trect.t..=trect.b {
                for tcol in trect.l..=trect.r {
    
                    if tcol < prev_trect.l || tcol > prev_trect.r ||
                        trow < prev_trect.t || trow > prev_trect.b {
                        
                        self.tight.insert(lcol, lrow, tcol, trow);
                    }
                }
            }
        }

    }

    pub fn print_cells(&self) {
        self.tight.print_cells();
        self.loose.print_cells();
    }


    pub fn print_agents(&self) {
        self.tight.print_agents(self);
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