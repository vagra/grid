use bevy::{prelude::*, reflect::TypeUuid};
use grid::ugrid::{UGrid, agent::Agent};
use crate::*;
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
pub struct UAgent(pub Agent);

#[derive(Component)]
pub struct UPos{
    pub x: i16,
    pub y: i16,
}


#[derive(Resource, Deref, DerefMut, TypeUuid)]
#[uuid = "e6470b99-0731-4836-902a-cf3e61bee04b"]
pub struct RUGrid(pub UGrid);

impl Default for RUGrid {
    fn default() -> Self {
        
        Self(UGrid::default())
    }
}


pub fn create_ugrid(
    mut commands: Commands
) {

    println!("create grid:");

    let mut grid = RUGrid::default();
    grid.init_test_data();

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


pub fn change_uagent(
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

