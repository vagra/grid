use bevy::{prelude::*, reflect::TypeUuid};
use grid::dgrid::{DGrid, agent::Agent};
use crate::input::*;
use super::*;

pub mod dagent;
pub mod lrect;
pub mod lcell;
pub mod tcell;

use {
    dagent::*,
    lrect::*,
    lcell::*,
    tcell::*,
};

pub const IDS: [u32; 9] = [
    101, 102, 103, 104, 105, 106, 107, 108, 109
];



#[derive(Component)]
pub struct LCol(pub u16);

#[derive(Component)]
pub struct LRow(pub u16);


#[derive(Component)]
pub struct TCol(pub u16);

#[derive(Component)]
pub struct TRow(pub u16);

#[derive(Component)]
pub struct DAgent(pub Agent);


#[derive(Component)]
pub struct DPos{
    pub x: i16,
    pub y: i16,
}


#[derive(Resource, Deref, DerefMut, TypeUuid)]
#[uuid = "e05ab7bd-6801-4105-b98d-97dfb9da1d7f"]
pub struct RDGrid(pub DGrid);

impl Default for RDGrid {
    fn default() -> Self {
        
        Self(DGrid::default())
    }
}

impl RDGrid {

    pub fn new() -> Self {

        Self(DGrid::new(4, 25, 10, 10))
    }
}


pub fn create_dgrid(
    mut commands: Commands
) {

    println!("create grid:");

    let mut grid = RDGrid::default();
    // grid.init_test_data();

    grid.insert(101, 23, 24, 3, 3);
    grid.insert(102, 12, 10, 5, 5);
    grid.insert(103, 6, 23, 7, 7);
    grid.insert(104, 40, 97,9, 9);
    grid.insert(105, -123, -432, 10, 10);
    grid.insert(106, -234, 324, 12, 12);
    grid.insert(107, 450, 123, 14, 14);
    grid.insert(108, 480, 170, 16, 16);
    grid.insert(109, 15, 27, 20, 20);

    // grid.remove(103, 6, 23);
    // grid.remove(106, -234, 324);
    // grid.remove(109, 15, 27);

    grid.optimize();

    create_tcells(&mut commands, &grid);
    create_lcells(&mut commands, &grid);
    create_lrects(&mut commands, &grid);
    create_dagents(&mut commands, &grid);

    commands.insert_resource(grid);

    let cmd:Cmd = Cmd{
        index: 0,
        dir: None,
    };
    commands.insert_resource(cmd);

    commands.insert_resource(NextState(Some(GameState::Playing)));

    println!("create grid done.");
}

pub fn many_create_dgrid(
    mut commands: Commands
) {

    println!("create many dgrid:");

    let mut grid = RDGrid::new();
    
    grid.insert_rand_data(AGENTS, MIN_HALF_SIZE, MAX_HALF_SIZE);

    grid.optimize();

    create_tcells(&mut commands, &grid);
    create_lcells(&mut commands, &grid);
    create_lrects(&mut commands, &grid);
    many_create_dagents(&mut commands, &grid);

    commands.insert_resource(grid);

    let cmd:Cmd = Cmd{
        index: 0,
        dir: None,
    };
    commands.insert_resource(cmd);

    commands.insert_resource(NextState(Some(GameState::Playing)));

    println!("create many dgrid done.");
}

pub fn keyboard_input(
    mut cmd: ResMut<Cmd>,
    input: Res<Input<KeyCode>>
) {

    let z = input.pressed(KeyCode::Z);
    let x = input.pressed(KeyCode::X);

    if z {
        cmd.index = (cmd.index + 1) % 9;
    }
    if x {
        cmd.index = (cmd.index + 8) % 9;
    }
    
    cmd.dir = key2dir(&input);
}