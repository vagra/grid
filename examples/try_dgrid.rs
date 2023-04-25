#![allow(dead_code)]
use grid::{
    dgrid::*,
};


fn main() {
    // test_tight_loose_print();
    // test_new();
    test_insert();
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

pub fn test_insert() {

    println!("\n------------------------------------------------");
    println!("test_insert");

    let mut grid = DGrid::default();

    grid.insert(101, 12, 34, 10, 10);
    grid.insert(102, 23, 56, 10, 10);
    grid.insert(103, 78, 12, 10, 10);
    grid.insert(104, 89, 65, 10, 10);

    grid.print_valid_cells();
}