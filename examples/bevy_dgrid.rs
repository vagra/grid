use bevy::prelude::*;


mod mods;

use mods::{*, clcell::*, clrect::*, ctcell::*, cagent::*};


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
            (create_grid).after(update)
            .run_if(in_state(GameState::Starting))
        )
        .add_system(
            (change_agent).after(update)
            .run_if(in_state(GameState::Playing))
        )
        .add_system(
            (move_agent).after(change_agent)
            .run_if(in_state(GameState::Playing))
        )
        .add_system(
            (update_tcells).after(move_agent)
            .run_if(in_state(GameState::Playing))
        )
        .add_system(
            (update_lcells).after(move_agent)
            .run_if(in_state(GameState::Playing))
        )
        .add_system(
            (update_lrects).after(optimize_grid)
            .run_if(in_state(GameState::Playing))
        )
        .run();
}

fn create_camera(mut commands: Commands) {

    commands.spawn(Camera2dBundle::default());
}


fn update() {

}