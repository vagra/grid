use std::mem;
use grid::{
    *, pool::*, items::*,
    ugrid::{*, agent::*}
};


fn main() {
    test_insert_remove();
    test_optimize();
    test_move_cell();
    test_pos2grid();
    test_pos2cell();
    test_find_in_cell();
    test_out_bounds_insert();
    test_out_bounds_remove();

    test_query(); 
    test_query_dirs();

    print_size();

    test_clear(); 
}


fn test_insert_remove() {
    println!("\n------------------------------------------------");
    println!("test_insert_remove");

    let mut grid = UGrid::default();

    grid.insert_test_data();
    
    grid.remove(107, 35, 35);
    grid.remove(109, 21, 23);
    
    grid.print_cells();
    grid.print_pool();
    grid.print_agents();
}


fn test_move_cell() {
    println!("\n------------------------------------------------");
    println!("test_move_cell");

    let mut grid = UGrid::default();

    grid.insert_test_data();

    grid.print_cells();
    grid.print_pool();
    grid.print_agents();

    grid.move_cell(107, 35, 35, 143, -165);
    grid.move_cell(106, 24, 62, 112, -123);
    grid.move_cell(106, 112, -123, 24, 62);
    
    grid.print_cells();
    grid.print_pool();
    grid.print_agents();
}

fn test_optimize() {
    println!("\n------------------------------------------------");
    println!("test_optimize");

    let mut grid = UGrid::default();

    grid.insert_test_data();
    
    grid.remove(107, 35, 35);
    grid.remove(109, 21, 23);

    grid.print_cells();
    grid.print_pool();
    grid.print_agents();

    grid.optimize();
    
    grid.print_cells();
    grid.print_pool();
    grid.print_agents();
}


fn test_query() {
    println!("\n------------------------------------------------");
    println!("test_query");

    let mut grid = UGrid::default();
    grid.insert_test_data();

    grid.insert(201, 25, 45);
    let vec = grid.query(25, 45, 201);
    grid.print_query(&vec);
}


fn test_query_dirs() {
    println!("\n------------------------------------------------");
    println!("test_query_dirs");

    let mut grid = UGrid::default();
    grid.insert_test_data();

    grid.insert(201, 25, 45);

    let vec = grid.query_dirs(25, 45, 201);
    println!("201: {:?}", &vec);

    let vec = grid.query_dirs(12, 23, 101);
    println!("101: {:?}", &vec);
    let vec = grid.query_dirs(-123, 223, 102);
    println!("102: {:?}", &vec);
    let vec = grid.query_dirs(-323, -123, 103);
    println!("103: {:?}", &vec);
    let vec = grid.query_dirs(123, -123, 104);
    println!("104: {:?}", &vec);
    let vec = grid.query_dirs(423, 223, 105);
    println!("105: {:?}", &vec);
    let vec = grid.query_dirs(24, 62, 106);
    println!("106: {:?}", &vec);
    let vec = grid.query_dirs(35, 35, 107);
    println!("107: {:?}", &vec);
    let vec = grid.query_dirs(42, 43, 108);
    println!("108: {:?}", &vec);
    let vec = grid.query_dirs(21, 23, 109);
    println!("109: {:?}", &vec);

}


fn test_pos2grid() {
    println!("\n------------------------------------------------");
    println!("test_pos2grid");

    let grid = UGrid::default();
    
    print!("{:?}\t", grid.pos2grid(-999, 599));
    print!("{:?}\t", grid.pos2grid(-1000, 600));
    println!("{:?}", grid.pos2grid(-1000, 600));
    
    print!("{:?}\t", grid.pos2grid(999, 599));
    print!("{:?}\t", grid.pos2grid(1000, 600));
    println!("{:?}", grid.pos2grid(1000, 600));
    
    print!("{:?}\t\t", grid.pos2grid(999, -599));
    print!("{:?}", grid.pos2grid(1000, -600));
    println!("{:?}", grid.pos2grid(1000, -600));
    
    print!("{:?}\t", grid.pos2grid(-999, -599));
    print!("{:?}\t", grid.pos2grid(-1000, -600));
    println!("{:?}", grid.pos2grid(-1000, -600));
}

fn test_pos2cell() {
    println!("\n------------------------------------------------");
    println!("test_pos2cell");

    let grid = UGrid::default();

    println!("{:?}", grid.pos2cell(-2000, 1600));
    println!("{:?}", grid.pos2cell(2000, 1600));
    println!("{:?}", grid.pos2cell(2000, -1600));
    println!("{:?}", grid.pos2cell(-2000, -1600));

    println!("{:?}", grid.pos2cell(-2000, 300));
    println!("{:?}", grid.pos2cell(2000, 300));
    println!("{:?}", grid.pos2cell(200, 1600));
    println!("{:?}", grid.pos2cell(200, -1600));

    println!("{:?}", grid.pos2cell(-528, 0));
    println!("{:?}", grid.pos2cell(-528, -0));
    println!("{:?}", grid.pos2cell(-527, -0));
}

fn test_find_in_cell() {
    println!("\n------------------------------------------------");
    println!("test_find_in_cell");

    let mut grid = UGrid::default();
    grid.insert_test_data();

    grid.insert(122, -528, 0);
    grid.print_cells();
    grid.print_pool();

    grid.move_cell(122, -528, 0, -528, -0);
    grid.print_cells();
    grid.print_pool();

    grid.move_cell(122, -528, -0, -527, -0);
    grid.print_cells();
    grid.print_pool();
}


fn test_out_bounds_insert() {
    println!("\n------------------------------------------------");
    println!("test_out_bounds_insert");

    let mut grid = UGrid::default();
    grid.insert_test_data();

    grid.print_cells();
    grid.print_pool();

    grid.insert(201, -2000, 1600);
    grid.insert(202, 2000, 1600);
    grid.insert(203, 2000, -1600);
    grid.insert(204, -2000, -1600);

    grid.insert(205, -2000, 300);
    grid.insert(206, 2000, 300);
    grid.insert(207, 200, 1600);
    grid.insert(208, 200, -1600);

    grid.print_cells();
    grid.print_pool();

}


fn test_out_bounds_remove() {
    println!("\n------------------------------------------------");
    println!("test_out_bounds_remove");

    let mut grid = UGrid::default();
    grid.insert_test_data();

    grid.print_cells();
    grid.print_pool();

    grid.insert(205, -2000, 300);
    grid.insert(206, 2000, 300);
    grid.insert(207, 200, 1600);
    grid.insert(208, 200, -1600);
    grid.print_cells();
    grid.print_pool();

    grid.remove(205, -2000, 300);
    grid.remove(208, 200, -1600);
    grid.print_cells();
    grid.print_pool();

}



fn print_size() {
    println!("\n------------------------------------------------");
    println!("print_size");

    let mut grid = UGrid::default();

    grid.insert_test_data();

    println!("size of Agent: {}", mem::size_of::<Agent>());
    println!("size of Items<Agent>: {}", mem::size_of::<Items<Agent>>());
    println!("size of Pool<Agent>: {}", mem::size_of::<Pool<Agent>>());
    println!("size of Grid: {}", mem::size_of::<UGrid>());

    println!("size of pool: {}", mem::size_of::<Agent>() * POOL_SIZE as usize);
}



fn test_clear() {
    println!("\n------------------------------------------------");
    println!("test_clear");

    let mut grid = UGrid::default();
    grid.insert_test_data();
    
    grid.print_cells();
    grid.print_pool();
    grid.print_agents();

    grid.clear();

    grid.print_cells();
    grid.print_pool();
    grid.print_agents();
}