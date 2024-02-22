use bevy::prelude::*;
use super::*;


pub fn keyboard_input(
    mut cmd: ResMut<Cmd>,
    input: Res<ButtonInput<KeyCode>>
) {

    let z = input.pressed(KeyCode::KeyZ);
    let x = input.pressed(KeyCode::KeyX);

    if z {
        cmd.index = (cmd.index + 1) % 9;
    }
    if x {
        cmd.index = (cmd.index + 8) % 9;
    }
    
    cmd.dir = key2dir(&input);
}

pub fn key2dir(input:&ButtonInput<KeyCode>) ->Option<usize> {

	let mut l = input.pressed(KeyCode::ArrowLeft) as usize;
    let mut r = input.pressed(KeyCode::ArrowRight) as usize;
    let mut u = input.pressed(KeyCode::ArrowUp) as usize;
    let mut d = input.pressed(KeyCode::ArrowDown) as usize;

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
