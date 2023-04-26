#![allow(dead_code)]
use grid::{
    pool::*,
    dgrid::agent::*,
};


fn main() {
    test_insert_erase_clear();
}

fn test_insert_erase_clear() {
    let mut lpool:Pool<Agent> = Pool::default();

    lpool.insert(Agent::new(101, 23, 54, 10, 10));
    lpool.insert(Agent::new(102, 56, 10, 10, 10));
    lpool.insert(Agent::new(103, 87, 23, 10, 10));
    lpool.insert(Agent::new(104, 40, 97, 10, 10));
    lpool.insert(Agent::new(105, -123, -432, 10, 10));
    lpool.insert(Agent::new(106, -234, 324, 10, 10));
    lpool.insert(Agent::new(107, 450, 123, 10, 10));
    lpool.insert(Agent::new(108, 480, 170, 10, 10));
    lpool.insert(Agent::new(109, 65, 87, 10, 10));
    
    lpool.print();

    lpool.erase(2);
    println!("erase 2");
    lpool.print();

    lpool.erase(0);
    println!("erase 0");
    lpool.print();

    lpool.erase(5);
    println!("erase 5");
    lpool.print();

    let mut index = lpool.insert(Agent::new(201, 12, 34, 10, 10));
    println!("insert {}", index);
    lpool.print();

    index = lpool.insert(Agent::new(202, 304, 234, 10, 10));
    println!("insert {}", index);
    lpool.print();

    index = lpool.insert(Agent::new(203, -493, -239, 20, 20));
    println!("insert {}", index);
    lpool.print();

    index = lpool.insert(Agent::new(204, 543, -143, 30, 30));
    println!("insert {}", index);
    lpool.print();

    println!("clear");
    lpool.clear();
    lpool.print();
}