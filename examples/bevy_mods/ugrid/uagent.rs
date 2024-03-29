use bevy::{prelude::*, sprite::Anchor};

use grid::{INVALID, ItemSpec, GridComm, ugrid::agent::*};
use crate::sprite::*;
use super::super::*;
use super::{*, mover::*};


#[derive(Bundle)]
pub struct UAgentBundle {
    pub id: UID,
    pub pos: UPos,
    pub mover: Mover,

    pub sprite: SpriteBundle,
}


impl UAgentBundle {

    pub fn new(agent:&Agent, size:i16, moving:bool) -> Self {

        Self {

            id: UID(agent.id),
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
                                z: 10.0}
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

    let mut index: u16;
    let mut agent: &Agent;
    for urow in 0..grid.0.rows {
        for ucol in 0..grid.0.cols {

            index = grid.0.cells[urow][ucol].head;
        
            while index != INVALID {

                agent = &grid.0.pool[index];

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

    let mut index: u16;
    let mut agent: &Agent;
    let mut camera: Camera2dBundle;
    for urow in 0..grid.0.rows {
        for ucol in 0..grid.0.cols {

            index = grid.0.cells[urow][ucol].head;
        
            while index != INVALID {

                agent = &grid.0.pool[index];

                if !agent.is_free() {
                    
                    commands.spawn(UAgentBundle::new(&agent, grid.0.agent_size, true));

                    if agent.id == MAIN_ID {

                        camera = Camera2dBundle::default();
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
        &UID,
        &mut UPos,
        &mut Sprite,
        &mut Transform
    )>,
    mut grid: ResMut<RUGrid>,
    cmd: Res<Cmd>,
) {

    let mut curr:UPos = UPos::default();
    let mut ids:Vec<u16>;

    for (uid, mut prev, mut sprite, mut transform) in query.iter_mut() {

        if uid.0 != IDS[cmd.index] {

            if sprite.color == CROSS_COLOR {
                sprite.color = AGENT_COLOR;
            }
            continue;
        }

        if let Some(dir) = cmd.dir {

            (prev.x, prev.y, curr.x, curr.y) = move_sprite(dir, AGENT_SPEED, &mut transform);

            grid.move_cell(uid.0, prev.x, prev.y, curr.x, curr.y);

            ids = grid.query( curr.x, curr.y, uid.0 );

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
        &UID,
        &mut UPos,
        &mut Mover,
        &mut Sprite,
        &mut Transform
    )>,
    mut grid: ResMut<RUGrid>,
    cmd: Res<Cmd>,
    time: Res<Time>,
) {

    let mut curr:UPos = UPos::default();
    let mut dirs:Vec<usize>;
    let mut ids:Vec<u16>;

    for (uid, mut prev, mut mover, mut sprite, mut transform) in query.iter_mut() {

        if uid.0 != MAIN_ID {

            mover.timer.tick(time.delta());

            if mover.timer.finished() {
                mover.timer.reset();

                mover.random();
            }

            if mover.pause {

                mover.pause = false;
                continue;
            }

            (prev.x, prev.y, curr.x, curr.y) = move_sprite(mover.direction, mover.speed, &mut transform);

            grid.move_cell(uid.0, prev.x, prev.y, curr.x, curr.y);

            if let Some(dir) = grid.out_bounds(curr.x, curr.y) {

                mover.back(dir);
                continue;
            }

            dirs = grid.query_dirs( curr.x, curr.y, uid.0 );

            if dirs.len() > 0 {
                mover.dodge(&dirs);
            }
            else {
                mover.stop();
            }

            continue;
        }

        if let Some(dir) = cmd.dir {

            (prev.x, prev.y, curr.x, curr.y) = move_sprite(dir, AGENT_SPEED, &mut transform);

            grid.move_cell(uid.0, prev.x, prev.y, curr.x, curr.y);

            ids = grid.query( curr.x, curr.y, uid.0 );

            if ids.len() > 0 {
                sprite.color = CROSS_COLOR;
            }
            else {
                sprite.color = AGENT_COLOR;
            }
        }
    }
}

