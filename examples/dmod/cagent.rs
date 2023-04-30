use bevy::{prelude::*, sprite::Anchor};
use grid::{INVALID, ItemSpec, dgrid::agent::*};
use super::*;


#[derive(Component)]
pub struct DAgent(pub Agent);

#[derive(Component)]
pub struct DPos{
    pub x: i16,
    pub y: i16,
}

#[derive(Bundle)]
pub struct CAgentBundle {
    pub agent: DAgent,
    pub pos: DPos,

    #[bundle]
    pub sprite: SpriteBundle,
}


impl CAgentBundle {

    pub fn new(agent:&Agent) -> Self {

        Self {

            agent: DAgent(*agent),
            pos: DPos{
                    x: agent.x,
                    y: agent.y
                },

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

                if !agent.is_free() {
                    
                    commands.spawn(CAgentBundle::new(&agent));
                }

                index = agent.next;

            }
        }
    }

    println!("done.");
}


pub fn move_agent(
    mut query: Query<(
        &DAgent,
        &mut DPos,
        &mut Sprite,
        &mut Transform
    )>,
    mut grid: ResMut<Grid>,
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

            grid.optimize();

            let ids = grid.query(
                x, y, agent.0.hw, agent.0.hh, agent.0.id
            );

            if ids.len() > 0 {
                sprite.color = CROSS_COLOR;
            }
            else {
                sprite.color = AGENT_COLOR;
            }
        }

        return;
    }
}

