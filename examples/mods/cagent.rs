use bevy::prelude::*;
use grid::{INVALID, ItemSpec};
use bevy_prototype_lyon::{prelude::*, shapes::*};
use super::*;



#[derive(Component)]
pub struct CAgent;


#[derive(Bundle)]
pub struct CAgentBundle {
    pub rect: CAgent,

    #[bundle]
    pub bundle: ShapeBundle,
}


impl CAgentBundle {

    pub fn new(x:i16, y:i16, hw:i16, hh:i16) -> Self {

        let shape = Rectangle {
            extents: Vec2::new((hw * 2) as f32, (hh * 2) as f32),
            origin: RectangleOrigin::TopLeft,
        };

        Self {

            rect: CAgent,

            bundle: ShapeBundle { 
                path: GeometryBuilder::build_as(&shape),
                transform: Transform::from_translation(
                    Vec3{
                        x: (x - hw) as f32,
                        y: (y + hh) as f32,
                        z: 4.0}
                ),
                ..default()
            }
            
        }
    }
}



pub fn create_agents(
    mut commands: Commands,
    grid: Res<Grid>,
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
                        agent.x, agent.y, agent.hw, agent.hh
                    ))
                    .insert(Fill::color(AGENT_COLOR))
                    .insert(Stroke::new(AGENT_BORDER, ALINE_WIDTH));
                }

            }
        }
    }

    commands.insert_resource(NextState(Some(GameState::Playing)));

    println!("\tdone.");
}