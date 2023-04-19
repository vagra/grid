use std::mem;
use grid::{ugrid::*, agent::*, pool::*, cells::*};


fn main() {
    test_insert_remove();
    test_move_cell();
    test_pos2grid();
    test_pos2cell();
    test_grid2pos();
    test_cell2pos();
    test_find_in_cell();
    test_out_bounds_insert();
    test_out_bounds_remove();

    test_query();
    test_dir_query();

    print_size();
}


pub fn test_insert_remove() {
    let mut grid = UGrid::default();

    grid.init_test_data();

    grid.print_cells();
    println!("{}", grid.cells[5][10].head);
    grid.print_agents(5, 10);
    
    grid.remove(107, 35, 35);
    grid.print_cells();
    println!("{}", grid.cells[5][10].head);
    grid.print_agents(5, 10);

    grid.remove(109, 21, 23);
    grid.print_cells();
    println!("{}", grid.cells[5][10].head);
    grid.print_agents(5, 10);
}


pub fn test_move_cell() {

    let mut grid = UGrid::default();

    grid.init_test_data();

    grid.print_cells();
    println!("{}", grid.cells[5][10].head);
    grid.print_agents(5, 10);

    grid.move_cell(107, 35, 35, 143, -165);
    grid.move_cell(106, 24, 62, 112, -123);
    grid.print_cells();
    println!("{}", grid.cells[5][10].head);
    grid.print_agents(5, 10);
    println!("{}", grid.cells[7][11].head);
    grid.print_agents(7, 11);

    grid.move_cell(106, 112, -123, 24, 62);
    grid.print_cells();
    println!("{}", grid.cells[5][10].head);
    grid.print_agents(5, 10);
    println!("{}", grid.cells[7][11].head);
    grid.print_agents(7, 11);

}


pub fn test_query() {

    let mut grid = UGrid::default();
    grid.init_test_data();

    grid.insert(201, 25, 45);
    let vec = grid.query(25, 45, 201);
    grid.print_query(&vec);
}


pub fn test_dir_query() {

    let mut grid = UGrid::default();
    grid.init_test_data();

    grid.insert(201, 25, 45);

    let mut vec:Vec<u16>;

    vec = grid.dir_query(0, 25, 45, 201);
    grid.print_dir_query(0, &vec);

    vec = grid.dir_query(1, 25, 45, 201);
    grid.print_dir_query(1, &vec);

    vec = grid.dir_query(2, 25, 45, 201);
    grid.print_dir_query(2, &vec);

    vec = grid.dir_query(3, 25, 45, 201);
    grid.print_dir_query(3, &vec);

    vec = grid.dir_query(4, 25, 45, 201);
    grid.print_dir_query(4, &vec);

    vec = grid.dir_query(5, 25, 45, 201);
    grid.print_dir_query(5, &vec);

    vec = grid.dir_query(6, 25, 45, 201);
    grid.print_dir_query(6, &vec);

    vec = grid.dir_query(7, 25, 45, 201);
    grid.print_dir_query(7, &vec);

}


pub fn test_pos2grid() {
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

pub fn test_pos2cell() {
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

pub fn test_grid2pos() {
    let grid = UGrid::default();

    println!("{:?}\t", grid.grid2pos(0, 0));
    println!("{:?}\t", grid.grid2pos(2000, 0));
    println!("{:?}\t", grid.grid2pos(2000, 1200));
    println!("{:?}\t", grid.grid2pos(0, 1200));

    println!();
}

pub fn test_cell2pos() {
    let grid = UGrid::default();

    println!("{:?}", grid.cell2pos(0, 0));
    println!("{:?}", grid.cell2pos(19, 0));
    println!("{:?}", grid.cell2pos(19, 11));
    println!("{:?}", grid.cell2pos(0, 11));
    println!("{:?}", grid.cell2pos(10, 6));
}

pub fn test_find_in_cell() {
    let mut grid = UGrid::default();
    grid.init_test_data();

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


pub fn test_out_bounds_insert() {
    let mut grid = UGrid::default();
    grid.init_test_data();

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


pub fn test_out_bounds_remove() {
    let mut grid = UGrid::default();
    grid.init_test_data();

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



pub fn print_size() {
    let mut grid = UGrid::default();

    grid.init_test_data();

    println!("size of Agent: {}", mem::size_of::<Agent>());
    println!("size of AgentList: {}", mem::size_of::<AgentList>());
    println!("size of Rows: {}", mem::size_of::<Rows>());
    println!("size of Cols: {}", mem::size_of::<Cols>());
    println!("size of Pool: {}", mem::size_of::<Pool>());
    println!("size of Grid: {}", mem::size_of::<UGrid>());

    println!("size of pool: {}", mem::size_of::<Agent>() * POOL_SIZE as usize);
}
