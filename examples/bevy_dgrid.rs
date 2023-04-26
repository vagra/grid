use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;


mod mods;

use mods::{*, clcell::*, clrect::*, ctcell::*, cagent::*};


const BG_COLOR: Color = Color::rgb(0.31, 0.47, 0.51);


#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
enum GameState {
    #[default]
    Starting,
    DrawTCell,
    DrawLCell,
    DrawLRect,
    DrawAgent,
    Playing,
}


fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(ClearColor(BG_COLOR))
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_state::<GameState>()
        .add_startup_system(create_camera)
        .add_system(update)
        .add_system(
            (create_grid).after(update)
            .run_if(in_state(GameState::Starting))
        )
        .add_system(
            (create_tcells).after(create_grid)
            .run_if(in_state(GameState::DrawTCell))
        )
        .add_system(
            (create_lcells).after(create_tcells)
            .run_if(in_state(GameState::DrawLCell))
        )
        .add_system(
            (create_lrects).after(create_lcells)
            .run_if(in_state(GameState::DrawLRect))
        )
        .add_system(
            (create_agents).after(create_lrects)
            .run_if(in_state(GameState::DrawAgent))
        )
        .run();
}

fn create_camera(mut commands: Commands) {

    commands.spawn(Camera2dBundle::default());
}


fn update() {

}