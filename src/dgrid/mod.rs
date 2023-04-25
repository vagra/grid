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
                half_tcols * 2, half_trows * 2, lcell_radius * factor),
            
            loose: Loose::new(
                half_tcols * factor * 2, half_trows * factor * 2, lcell_radius),
        }
    }


    pub fn insert(&mut self, id: u32, x:i16, y:i16, hw:i16, hh:i16) {

        let (lcol, lrow) = self.loose.insert(id, x, y, hw, hh);

        self.expand_aabb(lcol, lrow, x, y, hw, hh);
    }
    
    pub fn remove(&mut self, id: u32, x:i16, y:i16) {

        self.loose.remove(id, x, y);
    }

    

    pub fn expand_aabb(&mut self, lcol:u16, lrow:u16,
        x:i16, y:i16, hw:i16, hh:i16) {

        let lcell = &mut self.loose.cells[lrow][lcol];
        let prev_lrect = lcell.rect.clone();
        lcell.expand(x, y, hw, hh);

        let trect = self.tight.box2trect(x, y, hw, hh);

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


    pub fn print_valid_cells(&self) {

        for trow in 0..self.tight.rows {
            for tcol in 0..self.tight.cols {

                let tcell = self.tight.cells[trow][tcol];

                if tcell.lhead == INVALID {
                    continue;
                }

                let mut lindex = tcell.lhead;

                println!("tcell:({:3},{:3}) -> lhead:{:5}", tcol, trow, lindex);

                while lindex != INVALID {

                    let titem = self.tight.pool[lindex];

                    let lprev = lindex;
                    lindex = titem.next;

                    if !titem.is_free() {
                        print!("{:5}: ", lprev);
                        titem.print();

                        let lcell = self.loose.cells[titem.lrow][titem.lcol];

                        if lcell.head == INVALID {
                            continue;
                        }

                        let mut index = lcell.head;

                        println!("\tlcell:({:3},{:3}) -> head:{:5}",
                            titem.lcol, titem.lrow, index);
                        
                        while index != INVALID {
                            let agent = self.loose.pool[index];

                            let prev = index;
                            index = agent.next;

                            if !agent.is_free() {
                                print!("\t{:5}: ", prev);
                                agent.print();
                            }

                        }


                    }

                }
            }
        }
    }
}