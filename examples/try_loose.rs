use grid::dgrid::loose::*;


fn main() {
    test_new();
    test_insert_remove();
    test_pos2lcell();
    test_out_bounds_insert();
    test_out_bounds_remove();
    test_clear();
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
    loose.init_test_data();
    
    loose.print_cells();
    loose.print_pool();
    loose.print_agents();

    loose.remove(102, 12, 10);
    loose.remove(103, 6, 23);

    loose.print_cells();
    loose.print_pool();
    loose.print_agents();
}

fn test_pos2lcell() {
    println!("\n------------------------------------------------");
    println!("test_pos2lcell");

    let loose = Loose::default();

    println!("{:?}", loose.pos2lcell(-2000, 1600));
    println!("{:?}", loose.pos2lcell(2000, 1600));
    println!("{:?}", loose.pos2lcell(2000, -1600));
    println!("{:?}", loose.pos2lcell(-2000, -1600));

    println!("{:?}", loose.pos2lcell(-2000, 300));
    println!("{:?}", loose.pos2lcell(2000, 300));
    println!("{:?}", loose.pos2lcell(200, 1600));
    println!("{:?}", loose.pos2lcell(200, -1600));

}

fn test_out_bounds_insert() {
    println!("\n------------------------------------------------");
    println!("test_out_bounds_insert");

    let mut loose = Loose::default();
    loose.init_test_data();

    loose.insert(201, -2000, 1600, 10, 10);
    loose.insert(202, 2000, 1600, 10, 10);
    loose.insert(203, 2000, -1600, 10, 10);
    loose.insert(204, -2000, -1600, 10, 10);

    loose.insert(205, -2000, 300, 10, 10);
    loose.insert(206, 2000, 300, 10, 10);
    loose.insert(207, 200, 1600, 10, 10);
    loose.insert(208, 200, -1600, 10, 10);

    loose.print_cells();
    loose.print_pool();
    loose.print_agents();
}



fn test_out_bounds_remove() {

    println!("\n------------------------------------------------");
    println!("test_out_bounds_remove");

    let mut loose = Loose::default();
    loose.init_test_data();

    loose.insert(205, -2000, 300, 10, 10);
    loose.insert(206, 2000, 300, 10, 10);
    loose.insert(207, 200, 1600, 10, 10);
    loose.insert(208, 200, -1600, 10, 10);

    loose.print_cells();
    loose.print_pool();
    loose.print_agents();

    loose.remove(205, -2000, 300);
    loose.remove(208, 200, -1600);

    loose.print_cells();
    loose.print_pool();
    loose.print_agents();
}



fn test_clear() {
    println!("\n------------------------------------------------");
    println!("test_clear");

    let mut loose = Loose::default();
    loose.init_test_data();
    
    loose.print_cells();
    loose.print_pool();
    loose.print_agents();

    loose.clear();

    loose.print_cells();
    loose.print_pool();
    loose.print_agents();
}