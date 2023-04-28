use grid::dgrid::tight::*;


fn main() {
    test_new();
    test_insert_remove();
    test_lcell2tcell();
    test_box2trect();
    test_clear();
}



fn test_new() {
    println!("\n------------------------------------------------");
    println!("test_new");

    let tight = Tight::new(10, 6, 60, 2);
    tight.print_cells();
    tight.print_pool();
}



fn test_insert_remove() {
    println!("\n------------------------------------------------");
    println!("test_insert_remove");

    let mut tight = Tight::default();
    tight.init_test_data();
    
    tight.print_cells();
    tight.print_pool();
    tight.print_titems();

    tight.remove_lcell(5, 7);
    tight.remove_lcell(32, 17);
    tight.remove_lcell(47, 13);

    tight.print_cells();
    tight.print_pool();
    tight.print_titems();
}


fn test_lcell2tcell() {
    println!("\n------------------------------------------------");
    println!("test_lcell2tcell");

    let tight = Tight::default();

    println!("{:?}", tight.lcell2tcell(32, 17));
    println!("{:?}", tight.lcell2tcell(14, 7));
    println!("{:?}", tight.lcell2tcell(6, 8));

    println!("{:?}", tight.lcell2tcell(0, 0));
    println!("{:?}", tight.lcell2tcell(63, 0));
    println!("{:?}", tight.lcell2tcell(63, 35));
    println!("{:?}", tight.lcell2tcell(0, 35));
}

fn test_box2trect() {
    println!("\n------------------------------------------------");
    println!("test_box2trect");

    let tight = Tight::default();
    
    println!("{:?}", tight.box2trect(23, 24, 10, 10));
    println!("{:?}", tight.box2trect(12, 10, 10, 10));
    println!("{:?}", tight.box2trect(6, 23, 10, 10));
    println!("{:?}", tight.box2trect(40, 97, 10, 10));

}

fn test_clear() {
    println!("\n------------------------------------------------");
    println!("test_clear");

    let mut tight = Tight::default();
    tight.init_test_data();
    
    tight.print_cells();
    tight.print_pool();
    tight.print_titems();

    tight.clear();

    tight.print_cells();
    tight.print_pool();
    tight.print_titems();
}