use bevy::prelude::*;

mod bevy_mods;

use bevy_mods::{
    *, input::*,
    ugrid::{*, ucell::*, uagent::*},
};


const BG_COLOR: Color = Color::rgb(0.31, 0.47, 0.51);


fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(ClearColor(BG_COLOR))
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_systems(Startup, create_camera)
        .add_systems(Update, update)
        .add_systems(Update,
            (create_ugrid).run_if(in_state(GameState::Starting))
        )
        .add_systems(Update,
            (
                keyboard_input,
                move_uagent,
                update_ucells
            ).chain().run_if(in_state(GameState::Playing))
        )
        .run();
}

fn create_camera(mut commands: Commands) {

    commands.spawn(Camera2dBundle::default());
}


fn update() {

}


