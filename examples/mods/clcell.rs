use bevy::prelude::*;
use grid::{GridComm, INVALID};
use bevy_prototype_lyon::{prelude::*, shapes::*};
use super::*;


#[derive(Component)]
pub struct CLCell;


#[derive(Bundle)]
pub struct LCellBundle {
    pub rect: CLCell,

    #[bundle]
    pub bundle: ShapeBundle,
}

impl LCellBundle {

    pub fn new(x:i16, y:i16, size:u16) -> Self {

        let shape = Rectangle {
            extents: Vec2::new(size as f32, size as f32),
            origin: RectangleOrigin::TopLeft,
        };

        Self {

            rect: CLCell,

            bundle: ShapeBundle { 
                path: GeometryBuilder::build_as(&shape),
                transform: Transform::from_translation(
                    Vec3{x: x as f32, y: y as f32, z:2.0}
                ),
                ..default()
            }
            
        }
    }
}


pub fn create_lcells(
    mut commands: Commands,
    grid: Res<Grid>,
) {
    print!("create lcell...");

    for lrow in 0..grid.0.loose.rows {
        for lcol in 0..grid.0.loose.cols {
            let lcell = &grid.0.loose.cells[lrow][lcol];

            let gx = lcol * grid.0.loose.cell_size;
            let gy = lrow * grid.0.loose.cell_size;

            let (x, y) = grid.0.grid2pos(gx as i16, gy as i16);

            if lcell.head == INVALID {
                commands.spawn(LCellBundle::new(x, y, grid.0.loose.cell_size))
                .insert(Stroke::new(LCELL_BORDER, LLINE_WIDTH));
            }
            else {
                commands.spawn(LCellBundle::new(x, y, grid.0.loose.cell_size))
                .insert(Fill::color(LCELL_COLOR))
                .insert(Stroke::new(LCELL_BORDER, LLINE_WIDTH));
            }
        }

        // println!();
    }

    commands.insert_resource(NextState(Some(GameState::DrawLRect)));

    println!("\tdone.");

}