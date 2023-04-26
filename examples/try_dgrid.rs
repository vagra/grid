#![allow(dead_code)]
use grid::{
    dgrid::*,
};


fn main() {
    // test_tight_loose_print();
    // test_new();
    test_insert();
}


fn test_tight_loose_print() {
    println!("\n------------------------------------------------");
    println!("test_tight_loose_print");

    let grid = DGrid::default();

    println!("tight.cells:");
    grid.tight.print_cells();
    
    println!("loose.cells:");
    grid.loose.print_cells();
}


fn test_new() {
    println!("\n------------------------------------------------");
    println!("test_new");

    let grid = DGrid::new(2, 20, 5, 3);

    grid.print_cells();
}

fn test_insert() {

    println!("\n------------------------------------------------");
    println!("test_insert");

    let mut grid = DGrid::default();

    grid.insert(101, 23, 24, 10, 10);
    grid.insert(102, 12, 10, 10, 10);
    grid.insert(103, 6, 23, 10, 10);
    grid.insert(104, 40, 97, 10, 10);
    grid.insert(105, -123, -432, 10, 10);
    grid.insert(106, -234, 324, 10, 10);
    grid.insert(107, 450, 123, 10, 10);
    grid.insert(108, 480, 170, 10, 10);
    grid.insert(109, 15, 27, 10, 10);

    grid.loose.print_cells();
    grid.loose.print_pool();
    grid.tight.print_cells();
    grid.tight.print_pool();
    grid.print_agents();
}