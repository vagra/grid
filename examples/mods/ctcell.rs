use bevy::prelude::*;
use grid::{GridComm, INVALID};
use bevy_prototype_lyon::{prelude::*, shapes::*};
use super::*;


#[derive(Component)]
pub struct CTCell;


#[derive(Bundle)]
pub struct TCellBundle {
    pub rect: CTCell,

    #[bundle]
    pub bundle: ShapeBundle,
}

impl TCellBundle {

    pub fn new(x:i16, y:i16, size:u16) -> Self {

        let shape = Rectangle {
            extents: Vec2::new(size as f32, size as f32),
            origin: RectangleOrigin::TopLeft,
        };

        Self {

            rect: CTCell,

            bundle: ShapeBundle { 
                path: GeometryBuilder::build_as(&shape),
                transform: Transform::from_translation(
                    Vec3{x: x as f32, y: y as f32, z:1.0}
                ),
                ..default()
            }
            
        }
    }
}


pub fn create_tcells(
    mut commands: Commands,
    grid: Res<Grid>,
) {
    print!("create tcell...");

    for trow in 0..grid.0.tight.rows {
        for tcol in 0..grid.0.tight.cols {
            let tcell = &grid.0.tight.cells[trow][tcol];

            let gx = tcol * grid.0.tight.cell_size;
            let gy = trow * grid.0.tight.cell_size;

            let (x, y) = grid.0.grid2pos(gx as i16, gy as i16);
            // print!("({},{})({},{})  ", gx, gy, x, y);

            if tcell.lhead == INVALID {
                commands.spawn(TCellBundle::new(x, y, grid.0.tight.cell_size))
                .insert(Stroke::new(TCELL_BORDER, TLINE_WIDTH));
            }
            else {
                commands.spawn(TCellBundle::new(x, y, grid.0.tight.cell_size))
                .insert(Fill::color(TCELL_COLOR))
                .insert(Stroke::new(TCELL_BORDER, TLINE_WIDTH));
            }
        }

        // println!();
    }

    commands.insert_resource(NextState(Some(GameState::DrawLCell)));

    println!("\tdone.");

}