#![allow(dead_code)]
use bevy::prelude::*;

pub mod ugrid;
pub mod dgrid;


const FRAMES: u32 = 10000;
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
