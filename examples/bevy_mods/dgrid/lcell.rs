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

    pub fn new(grid:&DGrid, lcol:u16, lrow:u16) -> Self {

        let (x, y) = grid.loose.lcell2pos(lcol, lrow);

        Self {

            lcol: LCol(lcol),
            lrow: LRow(lrow),

            sprite: SpriteBundle { 
                sprite: Sprite {
                    color: LCELL_COLOR.clone(),
                    custom_size: Some(
                        Vec2::new(grid.loose.cell_size as f32, grid.loose.cell_size as f32)
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

            commands.spawn(LCellBundle::new(&grid.0, lcol, lrow));
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