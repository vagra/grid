use bevy::{prelude::*, reflect::TypeUuid};
use grid::dgrid::DGrid;
use crate::*;

pub mod clcell;
pub mod clrect;
pub mod ctcell;
pub mod cagent;

const LCELL_COLOR: Color = Color::rgba(0.0, 0.6, 0.0, 0.6);
const LRECT_COLOR: Color = Color::rgba(0.8, 0.8, 0.0, 0.6);
const TCELL_COLOR: Color = Color::rgba(0.6, 0.0, 0.0, 0.6);
const AGENT_COLOR: Color = Color::rgba(0.0, 0.0, 0.6, 0.6);
const CROSS_COLOR: Color = Color::rgba(1.0, 0.0, 0.0, 1.0);

const AGENT_SPEED: f32 = 2.0;

const SQR: f32 = 0.7071;

pub const VECTORES: [Vec2; 8] = [
	Vec2{ x: 0.0, y:-1.0 },
	Vec2{ x: SQR, y:-SQR },
	Vec2{ x: 1.0, y: 0.0 },
	Vec2{ x: SQR, y: SQR },
	Vec2{ x: 0.0, y: 1.0 },
	Vec2{ x:-SQR, y: SQR },
	Vec2{ x:-1.0, y: 0.0 },
	Vec2{ x:-SQR, y:-SQR },
];

pub const IDS: [u32; 9] = [
    101, 102, 103, 104, 105, 106, 107, 108, 109
];


#[derive(Resource, Deref, DerefMut, TypeUuid)]
#[uuid = "e05ab7bd-6801-4105-b98d-97dfb9da1d7f"]
pub struct Grid(pub DGrid);

impl Default for Grid {
    fn default() -> Self {
        
        Self(DGrid::default())
    }
}


#[derive(Resource, TypeUuid)]
#[uuid = "8ac2a2d9-92a4-4b40-b09e-1a810bd3b58d"]
pub struct Cmd{
    pub index: usize,
    pub dir: Option<usize>,
}


pub fn create_grid(
    mut commands: Commands
) {

    println!("create grid:");

    let mut grid = Grid::default();
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
    create_agents(&mut commands, &grid);

    commands.insert_resource(grid);

    let cmd:Cmd = Cmd{
        index: 0,
        dir: None,
    };
    commands.insert_resource(cmd);

    commands.insert_resource(NextState(Some(GameState::Playing)));

    println!("create grid done.");
}

pub fn optimize_grid(
    mut grid: ResMut<Grid>
) {
    grid.0.optimize();
}


pub fn change_agent(
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

    let l = input.pressed(KeyCode::Left);
    let r = input.pressed(KeyCode::Right);
    let u = input.pressed(KeyCode::Up);
    let d = input.pressed(KeyCode::Down);
    
    cmd.dir = key2dir(l, r, u, d);
}


fn key2dir(l:bool, r:bool, u:bool, d:bool) -> Option<usize> {
    let mut li = l as usize;
    let mut ri = r as usize;
    let mut ui = u as usize;
    let mut di = d as usize;

    if l && r {
        li = 0;
        ri = 0;
    }

    if u && d {
        ui = 0;
        di = 0;
    }

    let pos: usize = (di << 3) + (li << 2) + (ui << 1) + ri;

    match pos {
        //dlur
        0b0001 => Some(2),
        0b0010 => Some(4),
        0b0100 => Some(6),
        0b1000 => Some(0),
        0b0011 => Some(3),
        0b0110 => Some(5),
        0b1100 => Some(7),
        0b1001 => Some(1),
        _ => None,
    }

}