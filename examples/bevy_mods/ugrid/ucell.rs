use bevy::{prelude::*, sprite::Anchor};
use grid::{*, ugrid::ucell::*};
use super::super::*;
use super::*;


#[derive(Bundle)]
pub struct UCellBundle {
    pub lcol: UCol,
    pub lrow: URow,

    #[bundle]
    pub sprite: SpriteBundle,
}

impl UCellBundle {

    pub fn new(lcol:u16, lrow:u16, x:i16, y:i16, size:u16) -> Self {

        Self {

            lcol: UCol(lcol),
            lrow: URow(lrow),

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
    for lrow in 0..grid.0.rows {
        for lcol in 0..grid.0.cols {

            gx = lcol * grid.0.cell_size;
            gy = lrow * grid.0.cell_size;

            (x, y) = grid.0.grid2pos(gx as i16, gy as i16);

            commands.spawn(UCellBundle::new(lcol, lrow, x, y, grid.0.cell_size));
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
    for (lcol, lrow, mut visibility) in query.iter_mut() {
        ucell = &grid.0.cells[lrow.0][lcol.0];

        if ucell.head == INVALID {
            *visibility = Visibility::Hidden;
        }
        else {
            *visibility = Visibility::Visible;
        }
    }
}