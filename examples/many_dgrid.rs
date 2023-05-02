use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};

mod bevy_mods;

use bevy_mods::{
    *, info::*, camera::*,
    dgrid::{*, dagent::*, lcell::*, lrect::*, tcell::*}
};


const BG_COLOR: Color = Color::rgb(0.31, 0.47, 0.51);


fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(ClearColor(BG_COLOR))
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_state::<GameState>()
        .add_startup_system(create_info)
        .add_system(update)
        .add_system(
            (many_create_dgrid).after(update)
            .run_if(in_state(GameState::Starting))
        )
        .add_system(
            (keyboard_input).after(update)
            .run_if(in_state(GameState::Playing))
        )
        .add_system(
            (many_move_dagents).after(keyboard_input)
            .run_if(in_state(GameState::Playing))
        )
        .add_system(
            (move_camera).after(keyboard_input)
            .run_if(in_state(GameState::Playing))
        )
        .add_system(
            (update_tcells).after(many_move_dagents)
            .run_if(in_state(GameState::Playing))
        )
        .add_system(
            (update_lcells).after(many_move_dagents)
            .run_if(in_state(GameState::Playing))
        )
        .add_system(
            (update_lrects).after(many_move_dagents)
            .run_if(in_state(GameState::Playing))
        )
        .add_system(
            (update_info).after(update)
            .run_if(in_state(GameState::Playing))
        )
        .run();
}



fn update() {

}