use bevy::{prelude::*, sprite::Anchor};
use grid::{INVALID, ItemSpec, GridComm, dgrid::agent::*};
use super::super::*;
use super::{*, mover::Mover, sprite::*};


#[derive(Bundle)]
pub struct DAgentBundle {
    pub id: DID,
    pub pos: DPos,
    pub size: DSize,
    pub mover: Mover,

    #[bundle]
    pub sprite: SpriteBundle,
}


impl DAgentBundle {

    pub fn new(agent:&Agent, moving:bool) -> Self {

        Self {

            id: DID(agent.id),
            pos: DPos{
                    x: agent.x,
                    y: agent.y
                },
            size: DSize {
                    hw: agent.hw,
                    hh: agent.hh
                },
            mover:  if moving { Mover::new() }
                    else { Mover::default() },

            sprite: SpriteBundle { 
                sprite: Sprite {
                    color: AGENT_COLOR.clone(),
                    custom_size: Some(
                        Vec2::new((agent.hw * 2) as f32, (agent.hh * 2) as f32)
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



pub fn create_dagents(
    commands: &mut Commands,
    grid: &RDGrid,
) {
    print!("create agents...");

    for lrow in 0..grid.0.loose.rows {
        for lcol in 0..grid.0.loose.cols {

            let mut index = grid.0.loose.cells[lrow][lcol].head;
        
            while index != INVALID {

                let agent = grid.0.loose.pool[index];

                if !agent.is_free() {
                    
                    commands.spawn(DAgentBundle::new(&agent, false));
                }

                index = agent.next;

            }
        }
    }

    println!("done.");
}


pub fn many_create_dagents(
    commands: &mut Commands,
    grid: &RDGrid,
) {
    print!("create random moving dagents...");

    for lrow in 0..grid.0.loose.rows {
        for lcol in 0..grid.0.loose.cols {

            let mut index = grid.0.loose.cells[lrow][lcol].head;
        
            while index != INVALID {

                let agent = grid.0.loose.pool[index];

                if !agent.is_free() {
                    
                    commands.spawn(DAgentBundle::new(&agent, true));

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


pub fn move_dagent(
    mut query: Query<(
        &DID,
        &DSize,
        &mut DPos,
        &mut Sprite,
        &mut Transform
    )>,
    mut grid: ResMut<RDGrid>,
    cmd: Res<Cmd>,
) {

    let mut curr:DPos = DPos::default();
    let mut ids:Vec<u16>;

    for (did, size, mut prev, mut sprite, mut transform) in query.iter_mut() {

        if did.0 != IDS[cmd.index] {

            if sprite.color == CROSS_COLOR {

                sprite.color = AGENT_COLOR;
            }

            continue;
        }

        if let Some(dir) = cmd.dir {

            (prev.x, prev.y, curr.x, curr.y) = move_sprite(dir, AGENT_SPEED, &mut transform);

            grid.move_cell(did.0, prev.x, prev.y, curr.x, curr.y);

            grid.optimize();

            ids = grid.query( curr.x, curr.y, size.hw, size.hh, did.0 );

            if ids.len() > 0 {
                sprite.color = CROSS_COLOR;
            }
            else {
                sprite.color = AGENT_COLOR;
            }
        }
    }
}



pub fn many_move_dagents(
    mut query: Query<(
        &DID,
        &DSize,
        &mut DPos,
        &mut Mover,
        &mut Sprite,
        &mut Transform
    )>,
    mut grid: ResMut<RDGrid>,
    cmd: Res<Cmd>,
    time: Res<Time>,
) {

    let mut curr:DPos = DPos::default();
    let mut dirs:Vec<usize>;
    let mut ids:Vec<u16>;

    for (did, size, mut prev, mut mover, mut sprite, mut transform) in query.iter_mut() {

        if did.0 != MAIN_ID {

            mover.timer.tick(time.delta());

            if mover.timer.finished() {
                mover.timer.reset();

                mover.random();
            }

            if mover.pause {

                mover.pause = false;
                continue;
            }

            (prev.x, prev.y, curr.x, curr.y) = move_sprite(mover.dir, mover.speed, &mut transform);

            grid.move_cell(did.0, prev.x, prev.y, curr.x, curr.y);

            if let Some(dir) = grid.out_bounds(curr.x, curr.y) {

                mover.back(dir);
                continue;
            }

            dirs = grid.query_dirs( curr.x, curr.y, size.hw, size.hh, did.0 );

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

            grid.move_cell(did.0, prev.x, prev.y, curr.x, curr.y);

            ids = grid.query( curr.x, curr.y, size.hw, size.hh, did.0 );

            if ids.len() > 0 {
                sprite.color = CROSS_COLOR;
            }
            else {
                sprite.color = AGENT_COLOR;
            }
        }
    }

    grid.optimize();
}
