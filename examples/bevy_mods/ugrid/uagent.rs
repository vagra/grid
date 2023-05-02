use bevy::{prelude::*, sprite::Anchor};

use grid::{INVALID, ItemSpec, GridComm, ugrid::agent::*};
use super::super::*;
use super::{*, mover::Mover};


#[derive(Bundle)]
pub struct UAgentBundle {
    pub agent: UAgent,
    pub pos: UPos,
    pub mover: Mover,

    #[bundle]
    pub sprite: SpriteBundle,
}


impl UAgentBundle {

    pub fn new(agent:&Agent, size:i16, moving:bool) -> Self {

        if moving && agent.id == MAIN_ID {
            println!("main agent: ({},{})", agent.x, agent.y);
        }

        Self {

            agent: UAgent(*agent),
            pos: UPos{
                    x: agent.x,
                    y: agent.y
                },
            mover:  if moving { Mover::new() }
                    else { Mover::default() },

            sprite: SpriteBundle { 
                sprite: Sprite {
                    color: AGENT_COLOR.clone(),
                    custom_size: Some(
                        Vec2::new(size as f32, size as f32)
                    ),
                    anchor: Anchor::Center,
                    ..default()
                    }, 

                transform: Transform::from_translation(
                            Vec3{
                                x: agent.x as f32,
                                y: agent.y as f32,
                                z: 4.0}
                            ),
                ..default()
            }
            
        }
    }
}



pub fn create_uagents(
    commands: &mut Commands,
    grid: &RUGrid,
) {
    print!("create uagents...");

    for urow in 0..grid.0.rows {
        for ucol in 0..grid.0.cols {

            let mut index = grid.0.cells[urow][ucol].head;
        
            while index != INVALID {

                let agent = grid.0.pool[index];

                if !agent.is_free() {
                    
                    commands.spawn(UAgentBundle::new(&agent, grid.0.agent_size, false));
                }

                index = agent.next;

            }
        }
    }

    println!("done.");
}



pub fn many_create_uagents(
    commands: &mut Commands,
    grid: &RUGrid,
) {
    print!("create random moving uagents...");

    for urow in 0..grid.0.rows {
        for ucol in 0..grid.0.cols {

            let mut index = grid.0.cells[urow][ucol].head;
        
            while index != INVALID {

                let agent = grid.0.pool[index];

                if !agent.is_free() {
                    
                    commands.spawn(UAgentBundle::new(&agent, grid.0.agent_size, true));

                    if agent.id == MAIN_ID {

                        let mut camera = Camera2dBundle::default();
                        camera.transform.translation.x = agent.x as f32;
                        camera.transform.translation.y = agent.y as f32;

                        commands.spawn(camera);
                    }
                }

                index = agent.next;

            }
        }
    }

    println!("done.");
}


pub fn move_uagent(
    mut query: Query<(
        &UAgent,
        &mut UPos,
        &mut Sprite,
        &mut Transform
    )>,
    mut grid: ResMut<RUGrid>,
    cmd: Res<Cmd>,
) {

    for (agent, mut prev, mut sprite, mut transform) in query.iter_mut() {

        if agent.0.id != IDS[cmd.index] {

            if sprite.color == CROSS_COLOR {

                sprite.color = AGENT_COLOR;
            }

            continue;
        }

        if let Some(dir) = cmd.dir {

            prev.x = transform.translation.x as i16;
            prev.y = transform.translation.y as i16;
    
            let offset = VECTORES[dir];
            transform.translation.x += AGENT_SPEED * offset.x;
            transform.translation.y += AGENT_SPEED * offset.y;

            let x = transform.translation.x as i16;
            let y = transform.translation.y as i16;

            grid.move_cell(agent.0.id, prev.x, prev.y, x, y);

            let ids = grid.query( x, y, agent.0.id );

            if ids.len() > 0 {
                sprite.color = CROSS_COLOR;
            }
            else {
                sprite.color = AGENT_COLOR;
            }
        }
    }
}


pub fn many_move_uagents(
    mut query: Query<(
        &UAgent,
        &mut UPos,
        &mut Mover,
        &mut Sprite,
        &mut Transform
    )>,
    mut grid: ResMut<RUGrid>,
    cmd: Res<Cmd>,
    time: Res<Time>,
) {

    for (agent, mut prev, mut mover, mut sprite, mut transform) in query.iter_mut() {

        if agent.0.id != MAIN_ID {

            mover.timer.tick(time.delta());

            if mover.timer.finished() {
                mover.timer.reset();

                mover.random();
            }

            prev.x = transform.translation.x as i16;
            prev.y = transform.translation.y as i16;

            let offset = VECTORES[mover.dir];
            transform.translation.x += mover.speed * offset.x;
            transform.translation.y += mover.speed * offset.y;

            let x = transform.translation.x as i16;
            let y = transform.translation.y as i16;

            grid.move_cell(agent.0.id, prev.x, prev.y, x, y);

            if let Some(dir) = grid.out_bounds(x, y) {

                mover.back(dir);
                
                continue;
            }

            let ids = grid.dir_query( mover.dir as u8, x, y, agent.0.id );

            if ids.len() > 0 {
                mover.bump();
            }

            continue;
        }

        if let Some(dir) = cmd.dir {

            prev.x = transform.translation.x as i16;
            prev.y = transform.translation.y as i16;
    
            let offset = VECTORES[dir];
            transform.translation.x += AGENT_SPEED * offset.x;
            transform.translation.y += AGENT_SPEED * offset.y;

            let x = transform.translation.x as i16;
            let y = transform.translation.y as i16;

            grid.move_cell(agent.0.id, prev.x, prev.y, x, y);

            let ids = grid.query( x, y, agent.0.id );

            if ids.len() > 0 {
                sprite.color = CROSS_COLOR;
            }
            else {
                sprite.color = AGENT_COLOR;
            }
        }
    }
}

