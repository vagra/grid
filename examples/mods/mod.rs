use bevy::{prelude::*, reflect::TypeUuid};
use grid::dgrid::DGrid;
use crate::*;

pub mod clcell;
pub mod clrect;
pub mod ctcell;
pub mod cagent;

const LCELL_COLOR: Color = Color::rgba(0.0, 0.6, 0.0, 0.6);
const LRECT_COLOR: Color = Color::rgba(0.6, 0.6, 0.0, 0.6);
const TCELL_COLOR: Color = Color::rgba(0.6, 0.0, 0.0, 0.6);
const AGENT_COLOR: Color = Color::rgba(0.0, 0.0, 0.6, 0.6);

const AGENT_ID: u32 = 107;
const AGENT_SPEED: f32 = 5.0;

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


#[derive(Resource, Deref, DerefMut, TypeUuid)]
#[uuid = "e05ab7bd-6801-4105-b98d-97dfb9da1d7f"]
pub struct Grid(pub DGrid);

impl Default for Grid {
    fn default() -> Self {
        
        Self(DGrid::default())
    }
}


pub fn create_grid(
    mut commands: Commands
) {

    println!("create grid:");

    let mut grid = Grid::default();
    grid.init_test_data();

    // grid.remove(103, 6, 23);
    // grid.remove(106, -234, 324);
    // grid.remove(109, 15, 27);

    create_tcells(&mut commands, &grid);
    create_lcells(&mut commands, &grid);
    create_lrects(&mut commands, &grid);
    create_agents(&mut commands, &grid);

    commands.insert_resource(grid);

    commands.insert_resource(NextState(Some(GameState::Playing)));

    println!("create grid done.");
}