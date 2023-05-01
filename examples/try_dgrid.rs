#![allow(dead_code)]
use grid::{
    dgrid::{*, agent::*}
};


fn main() {
    test_new();
    test_insert();
    test_remove();
    test_move();
    test_optimize();
    test_in_grid();
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
    grid.insert_test_data();

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
    grid.insert_test_data();

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


fn test_move() {
    println!("\n------------------------------------------------");
    println!("test_move");

    let mut grid = DGrid::default();
    grid.insert_test_data();

    grid.move_cell(107, 450, 123, 470, 150);

    grid.loose.print_cells();
    grid.loose.print_pool();
    grid.loose.print_agents();

    grid.tight.print_cells();
    grid.tight.print_pool();
    grid.print_agents();
}



fn test_optimize() {
    println!("\n------------------------------------------------");
    println!("test_optimize");

    let mut grid = DGrid::default();
    grid.insert_test_data();

    grid.remove(103, 6, 23);
    grid.remove(106, -234, 324);
    grid.remove(109, 15, 27);

    grid.move_cell(107, 450, 123, 470, 150);

    grid.optimize();

    grid.loose.print_cells();
    grid.loose.print_pool();
    grid.loose.print_agents();
    
    grid.tight.print_cells();
    grid.tight.print_pool();
    grid.print_agents();
}

fn test_in_grid() {
    println!("\n------------------------------------------------");
    println!("test_in_grid");

    let grid = DGrid::default();

    let a = Agent::new(102, -970, 550, 10, 10);
    let b = Agent::new(102, 969, 550, 10, 10);
    let c = Agent::new(102, 969, -549, 10, 10);
    let d = Agent::new(102, -970, -549, 10, 10);

    let e = Agent::new(102, -971, 550, 10, 10);
    let f = Agent::new(102, -970, 551, 10, 10);

    let g = Agent::new(102, -971, 550, 10, 10);
    let h = Agent::new(102, -970, 551, 10, 10);

    let i = Agent::new(102, 970, -549, 10, 10);
    let j = Agent::new(102, 969, -550, 10, 10);

    let l = Agent::new(102, -971, -559, 10, 10);
    let m = Agent::new(102, -970, -550, 10, 10);

    println!("{}", a.in_grid(&grid));
    println!("{}", b.in_grid(&grid));
    println!("{}", c.in_grid(&grid));
    println!("{}", d.in_grid(&grid));
    println!("{}", e.in_grid(&grid));
    println!("{}", f.in_grid(&grid));
    println!("{}", g.in_grid(&grid));
    println!("{}", h.in_grid(&grid));
    println!("{}", i.in_grid(&grid));
    println!("{}", j.in_grid(&grid));
    println!("{}", l.in_grid(&grid));
    println!("{}", m.in_grid(&grid));

}