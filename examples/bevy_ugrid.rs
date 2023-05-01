use bevy::prelude::*;

mod bevy_mods;

use bevy_mods::ugrid::{
    *, 
    ucell::*, 
    uagent::*,
};


const BG_COLOR: Color = Color::rgb(0.31, 0.47, 0.51);


#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
enum GameState {
    #[default]
    Starting,
    Playing,
}


fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(ClearColor(BG_COLOR))
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .add_startup_system(create_camera)
        .add_system(update)
        .add_system(
            (create_ugrid).after(update)
            .run_if(in_state(GameState::Starting))
        )
        .add_system(
            (change_uagent).after(update)
            .run_if(in_state(GameState::Playing))
        )
        .add_system(
            (move_uagent).after(change_uagent)
            .run_if(in_state(GameState::Playing))
        )
        .add_system(
            (update_ucells).after(move_uagent)
            .run_if(in_state(GameState::Playing))
        )
        .run();
}

fn create_camera(mut commands: Commands) {

    commands.spawn(Camera2dBundle::default());
}


fn update() {

}


