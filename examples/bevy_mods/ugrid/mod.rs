use bevy::{prelude::*, reflect::TypePath};
use grid::ugrid::UGrid;
use super::*;

pub mod uagent;
pub mod ucell;

use {
    uagent::*,
    ucell::*
};

pub const IDS: [u32; 10] = [
    100, 101, 102, 103, 104, 105, 106, 107, 108, 109
];


#[derive(Component)]
pub struct UCol(pub u16);

#[derive(Component)]
pub struct URow(pub u16);

#[derive(Component)]
pub struct UID(pub u32);

#[derive(Component, Default)]
pub struct UPos{
    pub x: i16,
    pub y: i16,
}


#[derive(Resource, Deref, DerefMut, TypePath)]
pub struct RUGrid(pub UGrid);

impl Default for RUGrid {
    fn default() -> Self {
        
        Self(UGrid::default())
    }
}

impl RUGrid {

    pub fn new() -> Self {

        Self(UGrid::new(5, 20, 50, 30))
    }
}


pub fn create_ugrid(
    mut commands: Commands
) {

    println!("create grid:");

    let mut grid = RUGrid::default();
    grid.insert_test_data();

    create_ucells(&mut commands, &grid);
    create_uagents(&mut commands, &grid);

    commands.insert_resource(grid);

    let cmd:Cmd = Cmd{
        index: 0,
        dir: None,
    };
    commands.insert_resource(cmd);

    commands.insert_resource(NextState(Some(GameState::Playing)));

    println!("create grid done.");
}



pub fn many_create_ugrid(
    mut commands: Commands
) {

    println!("create many ugrid:");

    let mut grid = RUGrid::new();
    
    grid.insert_rand_data(AGENTS);

    create_ucells(&mut commands, &grid);
    many_create_uagents(&mut commands, &grid);

    commands.insert_resource(grid);

    let cmd:Cmd = Cmd{
        index: 0,
        dir: None,
    };
    commands.insert_resource(cmd);

    commands.insert_resource(NextState(Some(GameState::Playing)));

    println!("create many ugrid done.");
}
