use grid::{dgrid::{*, loose::*, tight::*}, GridComm};
use super::*;


#[derive(Component)]
pub struct LRect;


#[derive(Bundle)]
pub struct LRectBundle {
    pub rect: LRect,

    #[bundle]
    pub bundle: ShapeBundle,
}

impl LRectBundle {

    pub fn new(x:i16, y:i16, size:u16) -> Self {

        let shape = Rectangle {
            extents: Vec2::new(size as f32, size as f32),
            origin: RectangleOrigin::TopLeft,
        };

        Self {

            rect: LRect,

            bundle: ShapeBundle { 
                path: GeometryBuilder::build_as(&shape),
                transform: Transform::from_translation(
                    Vec3{x: x as f32, y: y as f32, z:0.0}
                ),
                ..default()
            }
            
        }
    }
}


pub fn crate_lrects(
    grid: &DGrid,
    mut commands: Commands
) {
    for lrow in 0..grid.loose.rows {
        for lcol in 0..grid.loose.cols {
            let lcell = &grid.loose.cells[lrow][lcol];

            let gx = lcol * grid.loose.cell_size;
            let gy = lrow * grid.loose.cell_size;

            let (x, y) = grid.grid2pos(gx as i16, gy as i16);

            commands.spawn(LRectBundle::new(x, y, grid.loose.cell_size))
                .insert(Fill::color(Color::CYAN))
                .insert(Stroke::new(Color::BLACK, 1.0));
        }
    }

}