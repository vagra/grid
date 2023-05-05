use bevy::{prelude::*, sprite::Anchor};
use grid::{*, dgrid::tcell::*};
use super::super::*;
use super::*;


#[derive(Bundle)]
pub struct TCellBundle {
    pub tcol: TCol,
    pub trow: TRow,

    #[bundle]
    pub sprite: SpriteBundle,
}

impl TCellBundle {

    pub fn new(tcol:u16, trow:u16, x:i16, y:i16, size:u16) -> Self {

        Self {

            tcol: TCol(tcol),
            trow: TRow(trow),

            sprite: SpriteBundle { 
                sprite: Sprite {
                    color: TCELL_COLOR.clone(),
                    custom_size: Some(
                        Vec2::new(size as f32, size as f32)
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

    let mut gx: u16;
    let mut gy: u16;
    let mut x: i16;
    let mut y: i16;
    for trow in 0..grid.0.tight.rows {
        for tcol in 0..grid.0.tight.cols {

            gx = tcol * grid.0.tight.cell_size;
            gy = trow * grid.0.tight.cell_size;

            (x, y) = grid.0.grid2pos(gx as i16, gy as i16);

            commands.spawn(TCellBundle::new(tcol, trow, x, y, grid.0.tight.cell_size));
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