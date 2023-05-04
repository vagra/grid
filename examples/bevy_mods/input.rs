use bevy::prelude::*;


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
