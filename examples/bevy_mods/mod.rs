#![allow(dead_code)]

use bevy::{prelude::*, reflect::TypeUuid};

pub mod dgrid;
pub mod ugrid;

pub mod mover;
pub mod camera;
pub mod info;
pub mod sprite;
pub mod input;


const TCELL_COLOR: Color = Color::rgba(0.3, 0.3, 0.3, 0.6);
const LCELL_COLOR: Color = Color::rgba(0.0, 0.6, 0.0, 0.6);
const LRECT_COLOR: Color = Color::rgba(0.8, 0.8, 0.0, 0.6);
const UCELL_COLOR: Color = Color::rgba(0.0, 0.6, 0.0, 0.6);

const AGENT_COLOR: Color = Color::rgba(0.0, 0.0, 0.6, 0.6);
const CROSS_COLOR: Color = Color::rgba(1.0, 0.0, 0.0, 1.0);

const AGENT_SPEED: f32 = 2.0;

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
const MIN_DURATION:f32 = 5.0;
const MAX_DURATION:f32 = 8.0;

const MIN_HALF_SIZE:i16 = 2;
const MAX_HALF_SIZE:i16 = 10;

const AGENTS: u32 = 21000;

pub const MAIN_ID: u32 = 0;


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
