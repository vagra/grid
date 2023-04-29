use bevy::{prelude::*, sprite::Anchor};
use grid::{INVALID, ItemSpec};
use super::*;


#[derive(Component)]
pub struct AIndex(pub u16);


#[derive(Component)]
pub struct ID(pub u32);


#[derive(Bundle)]
pub struct CAgentBundle {
    pub index: AIndex,
    pub id: ID,

    #[bundle]
    pub sprite: SpriteBundle,
}


impl CAgentBundle {

    pub fn new(index:u16, id:u32, x:i16, y:i16, hw:i16, hh:i16) -> Self {

        Self {

            index: AIndex(index),
            id: ID(id),

            sprite: SpriteBundle { 
                sprite: Sprite {
                    color: AGENT_COLOR.clone(),
                    custom_size: Some(
                        Vec2::new((hw * 2) as f32, (hh * 2) as f32)
                    ),
                    anchor: Anchor::Center,
                    ..default()
                    }, 
                transform: Transform::from_translation(
                    Vec3{
                        x: x as f32,
                        y: y as f32,
                        z: 4.0}
                ),
                ..default()
            }
            
        }
    }
}



pub fn create_agents(
    commands: &mut Commands,
    grid: &Grid,
) {
    print!("create agents...");

    for lrow in 0..grid.0.loose.rows {
        for lcol in 0..grid.0.loose.cols {

            let mut index = grid.0.loose.cells[lrow][lcol].head;
        
            while index != INVALID {

                let agent = grid.0.loose.pool[index];

                index = agent.next;

                if !agent.is_free() {
                    
                    commands.spawn(CAgentBundle::new(
                        index, agent.id, agent.x, agent.y, agent.hw, agent.hh
                    ));
                }

            }
        }
    }

    println!("done.");
}




pub fn update_agents(
    mut query: Query<(
        &AIndex,
        &mut Transform
    )>,
    grid: Res<Grid>,
) {

    for (index, mut transform) in query.iter_mut() {

        if index.0 == INVALID {
            continue;
        }

        let agent = grid.0.loose.pool[index.0];

        if !agent.is_free() {
            transform.translation.x = agent.x as f32;
            transform.translation.y = agent.y as f32;
        }
    }
}


pub fn move_agent(
    mut query: Query<(
        &ID,
        &mut Transform
    )>,
    mut grid: ResMut<Grid>,
    input: Res<Input<KeyCode>>
) {

    for (id, mut transform) in query.iter_mut() {

        if id.0 != AGENT_ID {
            continue;
        }
    
        let l = input.pressed(KeyCode::Left);
        let r = input.pressed(KeyCode::Right);
        let u = input.pressed(KeyCode::Up);
        let d = input.pressed(KeyCode::Down);
    
        if let Some(pos) = key2dir(l, r, u, d) {

            let prev_x = transform.translation.x;
            let prev_y = transform.translation.y;
    
            let offset = VECTORES[pos];
            transform.translation.x += AGENT_SPEED * offset.x;
            transform.translation.y += AGENT_SPEED * offset.y;

            grid.move_cell(
                id.0, prev_x as i16, prev_y as i16,
                transform.translation.x as i16,
                transform.translation.y as i16);
        }

        return;
    }
}


fn key2dir(l:bool, r:bool, u:bool, d:bool) -> Option<usize> {
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