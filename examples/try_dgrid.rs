#![allow(dead_code)]
use grid::{
    dgrid::*,
};


fn main() {
    // test_tight_loose_print();
    test_new();
}


pub fn test_tight_loose_print() {
    println!("\n------------------------------------------------");
    println!("test_tight_loose_print");

    let grid = DGrid::default();

    println!("tight.cells:");
    grid.tight.print_cells();
    
    println!("loose.cells:");
    grid.loose.print_cells();
}


pub fn test_new() {
    println!("\n------------------------------------------------");
    println!("test_new");

    let grid = DGrid::new(2, 20, 5, 3);

    grid.print_cells();
}

