#![allow(dead_code)]
use bevy::prelude::*;
use rand::{SeedableRng, rngs::StdRng};

pub mod ugrid;
pub mod dgrid;


const FRAMES: u32 = 100;
const MIN_SPEED: f32 = 0.5;
const MAX_SPEED: f32 = 3.0;
const MIN_DURATION: u16 = 50;
const MAX_DURATION: u16 = 100;
const MIN_RADIUS: i16 = 2;
const MAX_RADIUS: i16 = 10;


const DIRECTIONS: u8 = 8;

const SQR: f32 = 0.7071;

const VECTORES: [Vec2; 8] = [
	Vec2{ x: 0.0, y:-1.0 },
	Vec2{ x: SQR, y:-SQR },
	Vec2{ x: 1.0, y: 0.0 },
	Vec2{ x: SQR, y: SQR },
	Vec2{ x: 0.0, y: 1.0 },
	Vec2{ x:-SQR, y: SQR },
	Vec2{ x:-1.0, y: 0.0 },
	Vec2{ x:-SQR, y:-SQR },
];



pub fn init_seed() -> StdRng{

    let seed: [u8; 32] = [
        3, 42, 93, 129, 1, 85, 72, 42, 84, 23, 95, 212, 253, 10, 4, 2,
        34, 123, 98, 12, 234, 121, 23, 32, 87, 64, 234, 176, 13, 243, 76, 243
    ];

    StdRng::from_seed(seed)
}