use bevy::{prelude::*, sprite::Anchor};
use grid::{*, dgrid::lcell::*};
use super::super::*;
use super::*;


#[derive(Bundle)]
pub struct LCellBundle {
    pub lcol: LCol,
    pub lrow: LRow,

    #[bundle]
    pub sprite: SpriteBundle,
}

impl LCellBundle {

    pub fn new(lcol:u16, lrow:u16, x:i16, y:i16, size:u16) -> Self {

        Self {

            lcol: LCol(lcol),
            lrow: LRow(lrow),

            sprite: SpriteBundle { 
                sprite: Sprite {
                    color: LCELL_COLOR.clone(),
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


pub fn create_lcells(
    commands: &mut Commands,
    grid: &RDGrid,
) {
    print!("create lcell...");

    let mut gx: u16;
    let mut gy: u16;
    let mut x: i16;
    let mut y: i16;
    for lrow in 0..grid.0.loose.rows {
        for lcol in 0..grid.0.loose.cols {

            gx = lcol * grid.0.loose.cell_size;
            gy = lrow * grid.0.loose.cell_size;

            (x, y) = grid.0.grid2pos(gx as i16, gy as i16);

            commands.spawn(LCellBundle::new(lcol, lrow, x, y, grid.0.loose.cell_size));
        }
    }

    println!("\tdone.");

}



pub fn update_lcells(
    mut query: Query<(
        &LCol, &LRow,
        &mut Visibility
    )>,
    grid: Res<RDGrid>,
) {

    let mut lcell: &LCell;
    for (lcol, lrow, mut visibility) in query.iter_mut() {
        lcell = &grid.0.loose.cells[lrow.0][lcol.0];

        if lcell.head == INVALID {
            *visibility = Visibility::Hidden;
        }
        else {
            *visibility = Visibility::Visible;
        }
    }
}