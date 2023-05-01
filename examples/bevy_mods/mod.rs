#![allow(dead_code)]

use bevy::{prelude::*, reflect::TypeUuid};

pub mod dgrid;
pub mod ugrid;


const TCELL_COLOR: Color = Color::rgba(0.3, 0.3, 0.3, 0.6);
const LCELL_COLOR: Color = Color::rgba(0.0, 0.6, 0.0, 0.6);
const LRECT_COLOR: Color = Color::rgba(0.8, 0.8, 0.0, 0.6);
const UCELL_COLOR: Color = Color::rgba(0.0, 0.6, 0.0, 0.6);

const AGENT_COLOR: Color = Color::rgba(0.0, 0.0, 0.6, 0.6);
const CROSS_COLOR: Color = Color::rgba(1.0, 0.0, 0.0, 1.0);

const AGENT_SPEED: f32 = 2.0;

const SQR: f32 = 0.7071;

pub const VECTORES: [Vec2; 8] = [
	Vec2{ x: 0.0, y:-1.0 },
	Vec2{ x: SQR, y:-SQR },
	Vec2{ x: 1.0, y: 0.0 },
	Vec2{ x: SQR, y: SQR },
	Vec2{ x: 0.0, y: 1.0 },
	Vec2{ x:-SQR, y: SQR },
	Vec2{ x:-1.0, y: 0.0 },
	Vec2{ x:-SQR, y:-SQR },
];



#[derive(Resource, TypeUuid)]
#[uuid = "8ac2a2d9-92a4-4b40-b09e-1a810bd3b58d"]
pub struct Cmd{
    pub index: usize,
    pub dir: Option<usize>,
}



pub fn key2dir(l:bool, r:bool, u:bool, d:bool) -> Option<usize> {
    let mut li = l as usize;
    let mut ri = r as usize;
    let mut ui = u as usize;
    let mut di = d as usize;

    if l && r {
        li = 0;
        ri = 0;
    }

    if u && d {
        ui = 0;
        di = 0;
    }

    let pos: usize = (di << 3) + (li << 2) + (ui << 1) + ri;

    match pos {
        //dlur
        0b0001 => Some(2),
        0b0010 => Some(4),
        0b0100 => Some(6),
        0b1000 => Some(0),
        0b0011 => Some(3),
        0b0110 => Some(5),
        0b1100 => Some(7),
        0b1001 => Some(1),
        _ => None,
    }

}