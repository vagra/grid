use bevy::prelude::*;
use grid::INVALID;
use bevy_prototype_lyon::{prelude::*, shapes::*};
use super::*;


#[derive(Component)]
pub struct CLRect;


#[derive(Bundle)]
pub struct LRectBundle {
    pub rect: CLRect,

    #[bundle]
    pub bundle: ShapeBundle,
}

impl LRectBundle {

    pub fn new(l:i16, t:i16, r:i16, b:i16) -> Self {

        let shape = Rectangle {
            extents: Vec2::new((r-l+1) as f32, (t-b+1) as f32),
            origin: RectangleOrigin::TopLeft,
        };

        Self {

            rect: CLRect,

            bundle: ShapeBundle { 
                path: GeometryBuilder::build_as(&shape),
                transform: Transform::from_translation(
                    Vec3{
                        x: l as f32,
                        y: t as f32,
                        z: 3.0}
                ),
                ..default()
            }
            
        }
    }
}


pub fn create_lrects(
    mut commands: Commands,
    grid: Res<Grid>,
) {
    print!("create lrect...");

    for lrow in 0..grid.0.loose.rows {
        for lcol in 0..grid.0.loose.cols {
            let lcell = &grid.0.loose.cells[lrow][lcol];

            // print!("({},{})({},{})  ", gx, gy, x, y);

            if lcell.head != INVALID {
                commands.spawn(LRectBundle::new(
                    lcell.rect.l, lcell.rect.t,
                    lcell.rect.r, lcell.rect.b
                ))
                .insert(Fill::color(LRECT_COLOR))
                .insert(Stroke::new(LRECT_BORDER, LLINE_WIDTH));
            }
        }

        // println!();
    }

    commands.insert_resource(NextState(Some(GameState::Playing)));

    println!("\tdone.");
}