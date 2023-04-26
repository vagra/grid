use bevy::{prelude::*, reflect::TypeUuid};
use grid::dgrid::DGrid;
use crate::*;

pub mod clcell;
pub mod clrect;
pub mod ctcell;
pub mod cagent;

const LCELL_COLOR: Color = Color::rgba(0.0, 0.6, 0.0, 0.6);
const LCELL_BORDER: Color = Color::rgba(0.0, 1.0, 0.0, 0.6);
const LRECT_COLOR: Color = Color::rgba(0.6, 0.6, 0.0, 0.6);
const LRECT_BORDER: Color = Color::rgba(1.0, 1.0, 0.0, 0.6);
const TCELL_COLOR: Color = Color::rgba(0.6, 0.0, 0.0, 0.6);
const TCELL_BORDER: Color = Color::rgba(1.0, 0.0, 0.0, 0.6);
const AGENT_COLOR: Color = Color::rgba(0.0, 0.0, 0.6, 0.6);
const AGENT_BORDER: Color = Color::rgba(0.0, 0.0, 1.0, 0.6);
const LLINE_WIDTH: f32 = 0.5;
const TLINE_WIDTH: f32 = 1.0;
const ALINE_WIDTH: f32 = 0.5;


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

    print!("create grid...");

    let mut grid = Grid::default();

    grid.0.insert(101, 23, 24, 10, 10);
    grid.0.insert(102, 12, 10, 10, 10);
    grid.0.insert(103, 6, 23, 10, 10);
    grid.0.insert(104, 40, 97, 10, 10);
    grid.0.insert(105, -123, -432, 10, 10);
    grid.0.insert(106, -234, 324, 10, 10);
    grid.0.insert(107, 450, 123, 10, 10);
    grid.0.insert(108, 480, 170, 10, 10);
    grid.0.insert(109, 15, 27, 10, 10);

    commands.insert_resource(grid);

    commands.insert_resource(NextState(Some(GameState::DrawTCell)));

    println!("\tdone.");
}