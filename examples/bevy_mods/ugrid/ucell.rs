use bevy::{prelude::*, sprite::Anchor};
use grid::{*, ugrid::ucell::*};
use super::super::*;
use super::*;


#[derive(Bundle)]
pub struct UCellBundle {
    pub ucol: UCol,
    pub urow: URow,

    #[bundle]
    pub sprite: SpriteBundle,
}

impl UCellBundle {

    pub fn new(ucol:u16, urow:u16, x:i16, y:i16, size:u16) -> Self {

        Self {

            ucol: UCol(ucol),
            urow: URow(urow),

            sprite: SpriteBundle { 
                sprite: Sprite {
                    color: UCELL_COLOR.clone(),
                    custom_size: Some(
                        Vec2::new(size as f32, size as f32)
                    ),
                    anchor: Anchor::TopLeft,
                    ..default()
                    }, 
                transform: Transform::from_translation(
                    Vec3{x: x as f32, y: y as f32, z:2.0}
                ),
                ..default()
            }
            
        }
    }
}


pub fn create_ucells(
    commands: &mut Commands,
    grid: &RUGrid,
) {
    print!("create ucell...");

    let mut gx: u16;
    let mut gy: u16;
    let mut x: i16;
    let mut y: i16;
    for urow in 0..grid.0.rows {
        for ucol in 0..grid.0.cols {

            gx = ucol * grid.0.cell_size;
            gy = urow * grid.0.cell_size;

            (x, y) = grid.0.grid2pos(gx as i16, gy as i16);

            commands.spawn(UCellBundle::new(ucol, urow, x, y, grid.0.cell_size));
        }
    }

    println!("\tdone.");

}



pub fn update_ucells(
    mut query: Query<(
        &UCol, &URow,
        &mut Visibility
    )>,
    grid: Res<RUGrid>,
) {

    let mut ucell: &UCell;
    for (ucol, urow, mut visibility) in query.iter_mut() {
        ucell = &grid.0.cells[urow.0][ucol.0];

        if ucell.head == INVALID {
            *visibility = Visibility::Hidden;
        }
        else {
            *visibility = Visibility::Visible;
        }
    }
}