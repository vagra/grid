use bevy::prelude::*;


mod bevy_mods;

use bevy_mods::{
    *, input::*,
    dgrid::{*, dagent::*, lcell::*, lrect::*, tcell::*}
};


const BG_COLOR: Color = Color::rgb(0.31, 0.47, 0.51);


fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(ClearColor(BG_COLOR))
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_systems(Startup, create_camera)
        .add_systems(Update,
            (create_dgrid).run_if(in_state(GameState::Loading))
        )
        .add_systems(Update,
            (
                keyboard_input,
                move_dagent,
                (
                    update_tcells,
                    update_lcells,
                    update_lrects
                )
            ).run_if(in_state(GameState::Playing))
        )
        .run();
}

fn create_camera(mut commands: Commands) {

    commands.spawn(Camera2dBundle::default());
}
