use bevy::{prelude::*, sprite::Anchor};
use grid::{*, dgrid::tcell::*};
use super::super::*;
use super::*;


#[derive(Bundle)]
pub struct TCellBundle {
    pub tcol: TCol,
    pub trow: TRow,

    pub sprite: SpriteBundle,
}

impl TCellBundle {

    pub fn new(grid:&DGrid, tcol:u16, trow:u16) -> Self {

        let (x, y) = grid.tight.tcell2pos(tcol, trow);

        Self {

            tcol: TCol(tcol),
            trow: TRow(trow),

            sprite: SpriteBundle { 
                sprite: Sprite {
                    color: TCELL_COLOR.clone(),
                    custom_size: Some(
                        Vec2::new(grid.tight.cell_size as f32, grid.tight.cell_size as f32)
                    ),
                    anchor: Anchor::TopLeft,
                    ..default()
                    }, 
                transform: Transform::from_translation(
                    Vec3{x: x as f32, y: y as f32, z:1.0}
                ),
                ..default()
            }
            
        }
    }
}


pub fn create_tcells(
    commands: &mut Commands,
    grid: &RDGrid,
) {
    print!("create tcell...");

    for trow in 0..grid.0.tight.rows {
        for tcol in 0..grid.0.tight.cols {

            commands.spawn(TCellBundle::new(&grid.0, tcol, trow));
        }
    }

    println!("\tdone.");

}



pub fn update_tcells(
    mut query: Query<(
        &TCol, &TRow,
        &mut Visibility
    )>,
    grid: Res<RDGrid>,
) {

    let mut tcell: &TCell;
    for (tcol, trow, mut visibility) in query.iter_mut() {
        tcell = &grid.0.tight.cells[trow.0][tcol.0];

        if tcell.head == INVALID {
            *visibility = Visibility::Hidden;
        }
        else {
            *visibility = Visibility::Visible;
        }
    }
}