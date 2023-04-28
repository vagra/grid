#![allow(dead_code)]
use grid::{
    dgrid::*,
};


fn main() {
    // test_new();
    test_insert();
    // test_remove();
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
    grid.init_test_data();

    grid.loose.print_cells();
    grid.loose.print_pool();
    grid.loose.print_agents();

    grid.tight.print_cells();
    grid.tight.print_pool();
    grid.print_agents();
}

fn test_remove() {
    println!("\n------------------------------------------------");
    println!("test_remove");

    let mut grid = DGrid::default();
    grid.init_test_data();

    grid.remove(103, 6, 23);
    grid.remove(106, -234, 324);
    grid.remove(109, 15, 27);

    grid.loose.print_cells();
    grid.loose.print_pool();
    grid.loose.print_agents();
    
    grid.tight.print_cells();
    grid.tight.print_pool();
    grid.print_agents();
}