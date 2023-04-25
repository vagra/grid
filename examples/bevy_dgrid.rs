use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::*, shapes::*};
use grid::dgrid::DGrid;

mod mods;

use mods::*;


fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(create_camera)
        .add_startup_system(create_grid)
        .run();
}

fn create_camera(mut commands: Commands) {

    commands.spawn(Camera2dBundle::default());
}

fn create_grid(mut commands: Commands) {

    let grid = DGrid::default();

    crate_lrects(&grid, commands);    
}