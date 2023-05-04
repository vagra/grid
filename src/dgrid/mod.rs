pub mod agent;
pub mod rect;

pub mod titem;
pub mod tcell;
pub mod tight;

pub mod lcell;
pub mod loose;

use crate::{
    *, cells::*, pool::*, dpos::*,
    dgrid::{agent::*, tight::*, loose::*, rect::*, tcell::*, lcell::*, titem::*}
};
use grid_derive::GridComm;
use rand::Rng;

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

        self.loose.move_lcell(id, prev_x, prev_y, x, y);
    }


    pub fn query(&self, x:i16, y:i16, hw:i16, hh:i16, omit_id: u32) -> Vec<u16> {

        let trect = self.tight.box2trect(x, y, hw, hh);

        let tvec = self.query_titem_indices(&trect, x, y, hw, hh);

        let mut vec: Vec<u16> = Vec::new();
        let mut titem: &TItem;
        let mut lcell: &LCell;
        let mut index: u16;
        
        for (_, tindex) in tvec.iter().enumerate() {

            titem = &self.tight.pool[*tindex];

            lcell = &self.loose.cells[titem.lrow][titem.lcol];
            index = lcell.head;

            while index != INVALID {

                let agent = self.loose.pool[index];

                if agent.id != omit_id &&
                    agent.in_grid(self) &&
                    agent.box_cross(x, y, hw, hh) {

                    vec.push(index);
                }

                index = agent.next;
            }
        }

        vec
    }


    pub fn query_dirs(&self, x: i16, y: i16, hw:i16, hh:i16, omit_id: u32) -> Vec<usize> {

        let trect = self.tight.box2trect(x, y, hw, hh);

        let tvec = self.query_titem_indices(&trect, x, y, hw, hh);

        let mut vec: Vec<usize> = Vec::new();
        let mut dirs: [bool; 8] = [false; 8];
        let mut titem: &TItem;
        let mut lcell: &LCell;
        let mut index: u16;
        let mut agent: Agent;
        let mut dx: i16;
        let mut dy: i16;
        let mut sw: i16;
        let mut sh: i16;
        
        for (_, tindex) in tvec.iter().enumerate() {

            titem = &self.tight.pool[*tindex];

            lcell = &self.loose.cells[titem.lrow][titem.lcol];
            index = lcell.head;

            while index != INVALID {

                agent = self.loose.pool[index];

                if agent.id != omit_id &&
                    agent.in_grid(self) {

                    dx = agent.x - x;
                    dy = agent.y - y;
                    sw = agent.hw + hw;
                    sh = agent.hh + hh;

                    if dbox_cross(dx, dy, sw, sh) {

                        dpos_cross_dirs(&mut dirs, dx, dy);
                    }
                }

                index = agent.next;
            }
        }

        for i in 0..8 {
            if !dirs[i] {
                vec.push(i)
            }
        }

        vec
    }

    pub fn optimize(&mut self) {

        self.rebuild_loose();
        self.rebuild_tight();
    }

    fn rebuild_loose(&mut self) {

        self.loose.rebuild_rects();
    }

    fn rebuild_tight(&mut self) {

        self.tight.clear();
        
        for lrow in 0..self.loose.rows {
            for lcol in 0..self.loose.cols {

                let lcell = self.loose.cells[lrow][lcol];

                if lcell.head == INVALID {
                    continue;
                }

                let trect = self.tight.lrect2trect(&lcell.rect);

                for trow in trect.t..=trect.b {
                    for tcol in trect.l..=trect.r {

                        self.tight.insert(lcol, lrow, tcol, trow);
                    }
                }
            }
        }
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

    fn query_titem_indices(&self, trect:&TRect, x:i16, y:i16, hw:i16, hh:i16) -> Vec<u16> {

        let mut tvec: Vec<u16> = Vec::new();
        let mut index: u16;
        let mut tcell: &TCell;
        let mut titem: &TItem;
        let mut lcell: &LCell;

        for trow in trect.t..=trect.b {
            for tcol in trect.l..=trect.r {

                tcell = &self.tight.cells[trow][tcol];
                index = tcell.head;

                while index != INVALID {

                    titem = &self.tight.pool[index];

                    if !tvec.contains(&index) {

                        lcell = &self.loose.cells[titem.lrow][titem.lcol];

                        if lcell.rect.cross_box(x, y, hw, hh) {
                            
                            tvec.push(index);
                        }
                    }

                    index = titem.next;
                }

            }
        }

        tvec
    }

    pub fn gen_rand_box(&self, half_min:i16, half_max:i16) -> (i16, i16, i16, i16) {

        let mut rng = rand::thread_rng();
        
        (
            rng.gen_range( -self.half_width..self.half_width ),
            rng.gen_range( -self.half_height..self.half_height ),
            rng.gen_range( half_min..half_max ),
            rng.gen_range( half_min..half_max )
        )
    }

    pub fn print_cells(&self) {
        self.tight.print_cells();
        self.loose.print_cells();
    }


    pub fn print_agents(&self) {
        self.tight.print_agents(self);
    }


    pub fn insert_test_data(&mut self) {
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

    pub fn insert_rand_data(&mut self, count:u32, half_min:i16, half_max:i16) {

        for i in 0..count {

            let (x, y, hw, hh) = self.gen_rand_box(half_min, half_max);

            self.insert(i, x, y, hw, hh);
        }
    }
}