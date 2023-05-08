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

    pub fn new(grid:&UGrid, ucol:u16, urow:u16) -> Self {

        let (x, y) = grid.ucell2pos(ucol, urow);

        Self {

            ucol: UCol(ucol),
            urow: URow(urow),

            sprite: SpriteBundle { 
                sprite: Sprite {
                    color: UCELL_COLOR.clone(),
                    custom_size: Some(
                        Vec2::new(grid.cell_size as f32, grid.cell_size as f32)
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

    for urow in 0..grid.0.rows {
        for ucol in 0..grid.0.cols {

            commands.spawn(UCellBundle::new(&grid.0, ucol, urow));
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