use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};

mod bevy_mods;

use bevy_mods::{
    *, info::*, camera::*, input::*,
    ugrid::{*, ucell::*, uagent::*},
};


const BG_COLOR: Color = Color::rgb(0.31, 0.47, 0.51);


fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(ClearColor(BG_COLOR))
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .init_state::<GameState>()
        .add_systems(Startup, create_info)
        .add_systems(Update, update)
        .add_systems(Update,
            (many_create_ugrid).run_if(in_state(GameState::Starting))
        )
        .add_systems(Update,
            (
                keyboard_input, 
                many_move_uagents,
                update_ucells,
                move_camera,
                update_info
            ).chain().run_if(in_state(GameState::Playing))
        )
        .run();
}



fn update() {

}