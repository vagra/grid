use bevy::prelude::*;
use super::*;

pub fn move_sprite(
    dir: usize,
    speed: f32,
    transform: &mut Transform,
) -> (i16, i16, i16, i16) {

    let prev_x = transform.translation.x as i16;
    let prev_y = transform.translation.y as i16;

    let offset = VECTORES[dir];
    transform.translation.x += speed * offset.x;
    transform.translation.y += speed * offset.y;

    let curr_x = transform.translation.x as i16;
    let curr_y = transform.translation.y as i16;
    
    (prev_x, prev_y, curr_x, curr_y)
}