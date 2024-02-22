use grid::{
    *, pool::*, 
    dgrid::agent::*,
};


#[test]
fn insert_erase_works() {
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

    assert_eq!(lpool.size, 9);
    assert_eq!(lpool.first_free, INVALID);
    assert_eq!(lpool.capacity(), 9);

    lpool.erase(2);
    assert_eq!(lpool.size, 8);
    assert_eq!(lpool.first_free, 2);
    assert_eq!(lpool.capacity(), 9);
    assert_eq!(lpool.data[2].id, INACTIVE);

    lpool.erase(0);
    assert_eq!(lpool.size, 7);
    assert_eq!(lpool.first_free, 0);
    assert_eq!(lpool.capacity(), 9);
    assert_eq!(lpool.data[2].id, INACTIVE);
    assert_eq!(lpool.data[0].id, INACTIVE);
    assert_eq!(lpool.data[0].next_free, 2);

    lpool.erase(5);
    assert_eq!(lpool.size, 6);
    assert_eq!(lpool.first_free, 5);
    assert_eq!(lpool.capacity(), 9);
    assert_eq!(lpool.data[2].id, INACTIVE);
    assert_eq!(lpool.data[0].id, INACTIVE);
    assert_eq!(lpool.data[0].next_free, 2);
    assert_eq!(lpool.data[5].id, INACTIVE);
    assert_eq!(lpool.data[5].next_free, 0);

    let mut index = lpool.insert(Agent::new(201, 12, 34, 10, 10));
    assert_eq!(index, 5);
    assert_eq!(lpool.size, 7);
    assert_eq!(lpool.first_free, 0);
    assert_eq!(lpool.capacity(), 9);
    assert_eq!(lpool.data[2].id, INACTIVE);
    assert_eq!(lpool.data[0].id, INACTIVE);
    assert_eq!(lpool.data[0].next_free, 2);
    assert_eq!(lpool.data[5],
        Agent{ id:201, x:12, y:34, hw:10, hh:10, next: INVALID, next_free: INVALID}
    );

    index = lpool.insert(Agent::new(202, 304, 234, 10, 10));
    assert_eq!(index, 0);
    assert_eq!(lpool.size, 8);
    assert_eq!(lpool.first_free, 2);
    assert_eq!(lpool.capacity(), 9);
    assert_eq!(lpool.data[2].id, INACTIVE);
    assert_eq!(lpool.data[5],
        Agent{ id:201, x:12, y:34, hw:10, hh:10, next: INVALID, next_free: INVALID}
    );
    assert_eq!(lpool.data[0],
        Agent{ id:202, x:304, y:234, hw:10, hh:10, next: INVALID, next_free: INVALID}
    );

    index = lpool.insert(Agent::new(203, -493, -239, 20, 20));
    assert_eq!(index, 2);
    assert_eq!(lpool.size, 9);
    assert_eq!(lpool.first_free, INVALID);
    assert_eq!(lpool.capacity(), 9);
    assert_eq!(lpool.data[5],
        Agent{ id:201, x:12, y:34, hw:10, hh:10, next: INVALID, next_free: INVALID}
    );
    assert_eq!(lpool.data[0],
        Agent{ id:202, x:304, y:234, hw:10, hh:10, next: INVALID, next_free: INVALID}
    );
    assert_eq!(lpool.data[2],
        Agent{ id:203, x:-493, y:-239, hw:20, hh:20, next: INVALID, next_free: INVALID}
    );

    index = lpool.insert(Agent::new(204, 543, -143, 30, 30));
    assert_eq!(index, 9);
    assert_eq!(lpool.size, 10);
    assert_eq!(lpool.first_free, INVALID);
    assert_eq!(lpool.capacity(), 10);
    assert_eq!(lpool.data[9],
        Agent{ id:204, x:543, y:-143, hw:30, hh:30, next: INVALID, next_free: INVALID}
    );
}


#[test]
fn clear_works() {
    let mut lpool:Pool<Agent> = Pool::default();

    insert_some(&mut lpool, 10);
    assert_eq!(lpool.size, 10);
    assert_eq!(lpool.first_free, INVALID);
    assert_eq!(lpool.capacity(), 10);

    lpool.clear();
    assert_eq!(lpool.size, 0);
    assert_eq!(lpool.first_free, INVALID);
    assert_eq!(lpool.capacity(), 0);
}


#[test]
fn index_works() {
    let mut lpool:Pool<Agent> = Pool::default();
    lpool.insert(Agent::new(101, 123, 432, 10, 10));
    lpool.insert(Agent::new(102, -430, -234, 20, 20));

    let mut element = lpool[0];
    assert_eq!(element,
        Agent{id:101, x:123, y:432, hw:10, hh:10, next:INVALID, next_free:INVALID}
    );

    element = lpool[1];
    assert_eq!(element,
        Agent{id:102, x:-430, y:-234, hw:20, hh:20, next:INVALID, next_free:INVALID}
    );
}


#[test]
fn index_mut_works() {
    let mut lpool:Pool<Agent> = Pool::default();
    lpool.insert(Agent::new(101, 123, 432, 10, 10));
    lpool.insert(Agent::new(102, -430, -234, 20, 20));

    let element = &mut lpool[0];
    assert_eq!(*element,
        Agent{id:101, x:123, y:432, hw:10, hh:10, next:INVALID, next_free:INVALID}
    );

    element.x = 100;
    element.y = 200;
    assert_eq!(lpool[0],
        Agent{id:101, x:100, y:200, hw:10, hh:10, next:INVALID, next_free:INVALID}
    );
}



#[test]
fn after_construction_has_no_first_free() {
    let lpool:Pool<Agent> = Pool::default();
    assert_eq!(lpool.first_free, INVALID);
    assert_eq!(lpool.capacity(), 0);
}

#[test]
fn after_insertion_has_no_first_free() {
    let mut lpool:Pool<Agent> = Pool::default();
    let index = lpool.insert(Agent::new(101, 123, 432, 10, 10)); 
    assert_eq!(index, 0);
    assert_eq!(lpool.first_free, INVALID);
    assert_eq!(lpool.capacity(), 1);
}

#[test]
fn after_deletion_has_first_free() {
    let mut lpool:Pool<Agent> = Pool::default();
    let index = lpool.insert(Agent::new(101, 123, 432, 10, 10));
    assert_eq!(index, 0);

    lpool.erase(0);
    assert_eq!(lpool.first_free, 0);
    assert_eq!(lpool.capacity(), 1);
}

#[test]
fn insert_after_delete_has_no_first_free() {
    let mut lpool:Pool<Agent> = Pool::default();
    let index = lpool.insert(Agent::new(101, 123, 432, 10, 10));
    assert_eq!(index, 0);

    lpool.erase(0);
    lpool.insert(Agent::new(102, -430, -234, 20, 20));

    assert_eq!(lpool.first_free, INVALID);
    assert_eq!(lpool.capacity(), 1);
}

#[test]
fn first_free_points_to_last_erased_index() {
    let mut lpool:Pool<Agent> = Pool::default();
    insert_some(&mut lpool, 4);
    
    lpool.erase(0);
    lpool.erase(1);
    assert_eq!(lpool.first_free, 1);
    assert_eq!(lpool.capacity(), 4);
}

#[test]
fn erase_in_ascending_order() {
    let mut lpool:Pool<Agent> = Pool::default();
    insert_some(&mut lpool, 4);
    lpool.erase(0);
    lpool.erase(1);
    lpool.erase(2);
    lpool.erase(3);
    assert_eq!(lpool.first_free, 3);
    assert_eq!(lpool.capacity(), 4);
}

#[test]
fn erase_in_descending_order() {
    let mut lpool:Pool<Agent> = Pool::default();
    insert_some(&mut lpool, 4);
    lpool.erase(3);
    lpool.erase(2);
    lpool.erase(1);
    lpool.erase(0);
    assert_eq!(lpool.first_free, 0);
    assert_eq!(lpool.capacity(), 4);
}

#[test]
fn erase_in_mixed_order() {
    let mut lpool:Pool<Agent> = Pool::default();
    insert_some(&mut lpool, 4);
    lpool.erase(0);
    lpool.erase(3);
    lpool.erase(1);
    lpool.erase(2);
    assert_eq!(lpool.first_free, 2);
    assert_eq!(lpool.capacity(), 4);
}


fn insert_some(lpool: &mut Pool<Agent>, n: u16) {
    for i in 0..n {
        lpool.insert(Agent::new(
            (100 + i) as u32, i as i16, i as i16, 10i16, 10i16));
    }
}