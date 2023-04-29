use bevy::{prelude::*, sprite::Anchor};
use grid::INVALID;
use super::*;


#[derive(Component)]
pub struct CLRect;


#[derive(Bundle)]
pub struct LRectBundle {
    pub rect: CLRect,
    pub lcol: LCol,
    pub lrow: LRow,

    #[bundle]
    pub sprite: SpriteBundle,
}

impl LRectBundle {

    pub fn new(lcol:u16, lrow:u16, l:i16, t:i16, r:i16, b:i16) -> Self {

        Self {

            rect: CLRect,
            lcol: LCol(lcol),
            lrow: LRow(lrow),

            sprite: SpriteBundle { 
                sprite: Sprite {
                    color: LRECT_COLOR.clone(),
                    custom_size: Some(
                        Vec2::new((r-l+1) as f32, (t-b+1) as f32)
                    ),
                    anchor: Anchor::TopLeft,
                    ..default()
                    }, 
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
    commands: &mut Commands,
    grid: &Grid,
) {
    print!("create lrect...");

    for lrow in 0..grid.0.loose.rows {
        for lcol in 0..grid.0.loose.cols {
            let lcell = &grid.0.loose.cells[lrow][lcol];

            if lcell.head != INVALID {
                commands.spawn(LRectBundle::new(
                    lcol, lrow,
                    lcell.rect.l, lcell.rect.t,
                    lcell.rect.r, lcell.rect.b
                ));
            }
            else {
                commands.spawn(LRectBundle::new(
                    lcol, lrow,
                    0, 0, 0, 0
                ));
            }
        }
    }

    println!("\tdone.");
}



pub fn update_lrects(
    mut query: Query<
        (
            &LCol, &LRow,
            &mut Visibility,
            &mut Sprite,
            &mut Transform,
        ),
        With<CLRect>
    >,
    grid: Res<Grid>,
) {

    for (lcol, lrow, mut visibility, mut sprite, mut transform) in query.iter_mut() {
        let lcell = grid.0.loose.cells[lrow.0][lcol.0];

        if lcell.head == INVALID {
            *visibility = Visibility::Hidden;
        }
        else {
            *visibility = Visibility::Visible;

            sprite.custom_size = Some(Vec2::new(
                (lcell.rect.r - lcell.rect.l + 1) as f32,
                (lcell.rect.t - lcell.rect.b + 1) as f32,
            ));

            transform.translation.x = lcell.rect.l as f32;
            transform.translation.y = lcell.rect.t as f32;
        }

    }
}