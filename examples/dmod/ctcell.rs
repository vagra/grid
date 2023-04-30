use bevy::{prelude::*, sprite::Anchor};
use grid::{GridComm, INVALID};
use super::*;


#[derive(Component)]
pub struct TCol(pub u16);

#[derive(Component)]
pub struct TRow(pub u16);



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
    grid: &Grid,
) {
    print!("create tcell...");

    for trow in 0..grid.0.tight.rows {
        for tcol in 0..grid.0.tight.cols {

            let gx = tcol * grid.0.tight.cell_size;
            let gy = trow * grid.0.tight.cell_size;

            let (x, y) = grid.0.grid2pos(gx as i16, gy as i16);

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
    grid: Res<Grid>,
) {

    for (tcol, trow, mut visibility) in query.iter_mut() {
        let tcell = grid.0.tight.cells[trow.0][tcol.0];

        if tcell.head == INVALID {
            *visibility = Visibility::Hidden;
        }
        else {
            *visibility = Visibility::Visible;
        }
    }
}