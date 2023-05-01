use bevy::{prelude::*, sprite::Anchor};
use grid::{GridComm, INVALID};
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

    for lrow in 0..grid.0.loose.rows {
        for lcol in 0..grid.0.loose.cols {

            let gx = lcol * grid.0.loose.cell_size;
            let gy = lrow * grid.0.loose.cell_size;

            let (x, y) = grid.0.grid2pos(gx as i16, gy as i16);

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

    for (lcol, lrow, mut visibility) in query.iter_mut() {
        let lcell = grid.0.loose.cells[lrow.0][lcol.0];

        if lcell.head == INVALID {
            *visibility = Visibility::Hidden;
        }
        else {
            *visibility = Visibility::Visible;
        }
    }
}