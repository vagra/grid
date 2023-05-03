#![allow(dead_code)]

use bevy::{prelude::*, reflect::TypeUuid};

pub mod dgrid;
pub mod ugrid;
pub mod mover;
pub mod camera;
pub mod info;

const TCELL_COLOR: Color = Color::rgba(0.3, 0.3, 0.3, 0.6);
const LCELL_COLOR: Color = Color::rgba(0.0, 0.6, 0.0, 0.6);
const LRECT_COLOR: Color = Color::rgba(0.8, 0.8, 0.0, 0.6);
const UCELL_COLOR: Color = Color::rgba(0.0, 0.6, 0.0, 0.6);

const AGENT_COLOR: Color = Color::rgba(0.0, 0.0, 0.6, 0.6);
const CROSS_COLOR: Color = Color::rgba(1.0, 0.0, 0.0, 1.0);

const AGENT_SPEED: f32 = 4.0;

const SQR: f32 = 0.7071;

const DIRECTIONS:usize = 8;

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

const MIN_SPEED:f32 = 0.5;
const MAX_SPEED:f32 = 2.0;
const MIN_DURATION:f32 = 2.0;
const MAX_DURATION:f32 = 8.0;

const MIN_HALF_SIZE:i16 = 2;
const MAX_HALF_SIZE:i16 = 10;

const BUMP_DELAY:usize = 10;

const AGENTS: u32 = 15000;

pub const MAIN_ID: u32 = 101;


#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
pub enum GameState {
    #[default]
    Starting,
    Playing,
}



#[derive(Resource, TypeUuid)]
#[uuid = "8ac2a2d9-92a4-4b40-b09e-1a810bd3b58d"]
pub struct Cmd{
    pub index: usize,
    pub dir: Option<usize>,
}


pub fn key2dir(input:&Input<KeyCode>) ->Option<usize> {

	let mut l = input.pressed(KeyCode::Left) as usize;
    let mut r = input.pressed(KeyCode::Right) as usize;
    let mut u = input.pressed(KeyCode::Up) as usize;
    let mut d = input.pressed(KeyCode::Down) as usize;

    if l > 0 && r > 0 {
        l = 0;
        r = 0;
    }

    if u > 0 && d > 0 {
        u = 0;
        d = 0;
    }

    let pos: usize = (d << 3) + (l << 2) + (u << 1) + r;

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
