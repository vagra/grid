use bevy::prelude::*;
use super::*;


pub fn move_camera(
    mut query: Query<
        &mut Transform,
        With<Camera>,
    >,
    cmd: Res<Cmd>,
) {
    let mut transform = query.get_single_mut()
                        .expect("error: camera not found.");

    if let Some(dir) = cmd.dir {

        let offset = VECTORES[dir];
        transform.translation.x += AGENT_SPEED * offset.x;
        transform.translation.y += AGENT_SPEED * offset.y;
    }
    
}