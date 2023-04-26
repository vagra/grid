#![allow(dead_code)]
use grid::{
    pool::*,
    dgrid::{agent::*, loose::*},
};


fn main() {
    test_new();
    test_insert_remove();
}


fn test_new() {
    println!("\n------------------------------------------------");
    println!("test_new");

    let loose = Loose::new(10, 6, 15);
    loose.print_cells();
    loose.print_pool();
}



fn test_insert_remove() {
    println!("\n------------------------------------------------");
    println!("test_insert_remove");

    let mut loose = Loose::default();

    loose.insert(101, 23, 24, 10, 10);
    loose.insert(102, 12, 10, 10, 10);
    loose.insert(103, 6, 23, 10, 10);
    loose.insert(104, 40, 97, 10, 10);
    loose.insert(105, -123, -432, 10, 10);
    loose.insert(106, -234, 324, 10, 10);
    loose.insert(107, 450, 123, 10, 10);
    loose.insert(108, 480, 170, 10, 10);
    let (lcol, lrow) = loose.insert(109, 15, 27, 10, 10);
    
    loose.print_cells();
    loose.print_pool();
    loose.print_agents(lrow, lcol)
}