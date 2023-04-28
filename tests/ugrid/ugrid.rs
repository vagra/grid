use grid::{*, ugrid::{*, agent::*}};

#[test]
fn default_work() {
    let grid = UGrid::default();

    assert_eq!(grid.pool.size, 0);
    assert_eq!(grid.cells.len(), grid.rows);
    assert_eq!(grid.cells[0].len(), grid.cols);
}

#[test]
fn insert_work() {
    let mut grid = UGrid::default();

    grid.init_test_data();

    assert_eq!(grid.cells[3][8].head, 2);
    assert_eq!(grid.cells[3][14].head, 5);
    assert_eq!(grid.cells[5][10].head, 9);
    assert_eq!(grid.cells[7][6].head, 3);
    assert_eq!(grid.cells[7][11].head, 4);

    assert_eq!(grid.pool[9],
        Agent{id:109, x:21, y:23, next:8, ..Default::default()}
    );
    assert_eq!(grid.pool[8],
        Agent {id:108, x:42, y:43, next:7, ..Default::default()}
    );
    assert_eq!(grid.pool[7],
        Agent{id:107, x:35, y:35, next:6, ..Default::default()}
    );
    assert_eq!(grid.pool[6],
        Agent {id:106, x:24, y:62, next:1, ..Default::default()}
    );
    assert_eq!(grid.pool[1],
        Agent{id:101, x:12, y:23, next:0, ..Default::default()}
    );
    assert_eq!(grid.pool[0],
        Agent{id:100, x:54, y:29, ..Default::default()}
    )
}

#[test]
fn index_work() {
    let mut grid = UGrid::default();

    grid.insert(101, 12, 98);
    grid.insert(102, 23, 76);
    grid.insert(103, 34, 65);

    assert_eq!(grid[(5, 10)],
        Agent{id:103, x:34, y:65, next:1, ..Default::default()}
    );
    assert_eq!(grid.pool[grid[(5, 10)].next],
        Agent{id:102, x:23, y:76, next:0, ..Default::default()}
    );
    assert_eq!(grid.pool[grid.pool[grid[(5, 10)].next].next],
        Agent{id:101, x:12, y:98, ..Default::default()}
    );

}

#[test]
fn remove_work() {

    let mut grid = UGrid::default();

    grid.init_test_data();

    grid.remove(107, 35, 35);
    grid.remove(109, 21, 23);

    assert_eq!(grid.cells[3][8].head, 2);
    assert_eq!(grid.cells[3][14].head, 5);
    assert_eq!(grid.cells[5][10].head, 8);
    assert_eq!(grid.cells[7][6].head, 3);
    assert_eq!(grid.cells[7][11].head, 4);

    assert_eq!(grid.pool[8],
        Agent {id:108, x:42, y:43, next:6, ..Default::default()}
    );
    assert_eq!(grid.pool[6],
        Agent {id:106, x:24, y:62, next:1, ..Default::default()}
    );
    assert_eq!(grid.pool[1],
        Agent{id:101, x:12, y:23, next:0, ..Default::default()}
    );
    assert_eq!(grid.pool[0],
        Agent{id:100, x:54, y:29, ..Default::default()}
    );

}

#[test]
fn move_cell_work() {
    let mut grid = UGrid::default();

    grid.init_test_data();

    grid.move_cell(107, 35, 35, 143, -165);
    grid.move_cell(106, 24, 62, 112, -123);
    
    assert_eq!(grid.cells[5][10].head, 9);
    assert_eq!(grid.cells[7][11].head, 6);

    assert_eq!(grid.pool[9],
        Agent{id:109, x:21, y:23, next:8, ..Default::default()}
    );
    assert_eq!(grid.pool[8],
        Agent{id:108, x:42, y:43, next:1, ..Default::default()}
    );
    assert_eq!(grid.pool[1],
        Agent{id:101, x:12, y:23, next:0, ..Default::default()}
    );
    assert_eq!(grid.pool[0],
        Agent{id:100, x:54, y:29, ..Default::default()}
    );

    assert_eq!(grid.pool[6],
        Agent{id:106, x:112, y:-123, next:7, ..Default::default()}
    );
    assert_eq!(grid.pool[7],
        Agent{id:107, x:143, y:-165, next:4, ..Default::default()}
    );
    assert_eq!(grid.pool[4],
        Agent{id:104, x:123, y:-123, ..Default::default()}
    );

    grid.move_cell(106, 112, -123, 24, 62);

    assert_eq!(grid.cells[5][10].head, 6);
    assert_eq!(grid.cells[7][11].head, 7);

    assert_eq!(grid.pool[6],
        Agent{id:106, x:24, y:62, next:9, ..Default::default()}
    );
    assert_eq!(grid.pool[9],
        Agent{id:109, x:21, y:23, next:8, ..Default::default()}
    );
    assert_eq!(grid.pool[8],
        Agent{id:108, x:42, y:43, next:1, ..Default::default()}
    );
    assert_eq!(grid.pool[1],
        Agent{id:101, x:12, y:23, next:0, ..Default::default()}
    );
    assert_eq!(grid.pool[0],
        Agent{id:100, x:54, y:29, ..Default::default()}
    );

    assert_eq!(grid.pool[7],
        Agent{id:107, x:143, y:-165, next:4, ..Default::default()}
    );
    assert_eq!(grid.pool[4],
        Agent{id:104, x:123, y:-123, ..Default::default()}
    );


}


#[test]
fn query_work() {
    let mut grid = UGrid::default();

    grid.init_test_data();

    grid.insert(201, 38, 39);
    let vec = grid.query(38, 39, 201);

    assert_eq!(vec.len(), 4);
    assert_eq!(vec, [9u16, 8u16, 7u16, 0u16]);
}


#[test]
fn dir_query_work() {
    let mut grid = UGrid::default();

    grid.init_test_data();

    grid.insert(201, 25, 45);
    let mut vec:Vec<u16>;

    vec = grid.dir_query(0, 25, 45, 201);
    assert_eq!(vec.len(), 1);
    assert_eq!(vec, [7u16]);

    vec = grid.dir_query(1, 25, 45, 201);
    assert_eq!(vec.len(), 2);
    assert_eq!(vec, [8u16, 7u16]);

    vec = grid.dir_query(2, 25, 45, 201);
    assert_eq!(vec.len(), 2);
    assert_eq!(vec, [8u16, 7u16]);

    vec = grid.dir_query(3, 25, 45, 201);
    assert_eq!(vec.len(), 0);

    vec = grid.dir_query(4, 25, 45, 201);
    assert_eq!(vec.len(), 1);
    assert_eq!(vec, [6u16]);

    vec = grid.dir_query(5, 25, 45, 201);
    assert_eq!(vec.len(), 1);
    assert_eq!(vec, [6u16]);

    vec = grid.dir_query(6, 25, 45, 201);
    assert_eq!(vec.len(), 0);

    vec = grid.dir_query(7, 25, 45, 201);
    assert_eq!(vec.len(), 0);
}

#[test]
fn in_grid_work() {

    let mut grid = UGrid::default();

    grid.init_test_data();

    assert!(grid.in_grid(-1000, 600));
    assert!(!grid.in_grid(-1001, 600));
    assert!(!grid.in_grid(-1000, 601));

    assert!(grid.in_grid(999, 600));
    assert!(!grid.in_grid(1000, 600));
    assert!(!grid.in_grid(999, 601));

    assert!(grid.in_grid(999, -599));
    assert!(!grid.in_grid(1000, -599));
    assert!(!grid.in_grid(999, -600));

    assert!(grid.in_grid(-1000, -599));
    assert!(!grid.in_grid(-1001, -599));
    assert!(!grid.in_grid(-1000, -600));
}


#[test]
fn in_cell_work() {

    let mut grid = UGrid::default();

    grid.init_test_data();

    assert!(grid.in_cell(108, 5, 10));
    assert!(grid.in_cell(106, 5, 10));
    assert!(grid.in_cell(101, 5, 10));


    assert!(grid.in_cell(104, 7, 11));
    assert!(!grid.in_cell(107, 7, 11));
    
}


#[test]
fn pos2cell_work() {

    let grid = UGrid::default();
    
    assert_eq!((0, 0), grid.pos2cell(-2000, 1600));
    assert_eq!((19, 0), grid.pos2cell(2000, 1600));
    assert_eq!((19, 11), grid.pos2cell(2000, -1600));
    assert_eq!((0, 11), grid.pos2cell(-2000, -1600));

    assert_eq!((0, 3), grid.pos2cell(-2000, 300));
    assert_eq!((19, 3), grid.pos2cell(2000, 300));
    assert_eq!((12, 0), grid.pos2cell(200, 1600));
    assert_eq!((12, 11), grid.pos2cell(200, -1600));
}

#[test]
fn out_bounds_insert_work() {
    let mut grid = UGrid::default();
    grid.init_test_data();

    grid.insert(201, -2000, 1600);
    grid.insert(202, 2000, 1600);
    grid.insert(203, 2000, -1600);
    grid.insert(204, -2000, -1600);

    grid.insert(205, -2000, 300);
    grid.insert(206, 2000, 300);
    grid.insert(207, 200, 1600);
    grid.insert(208, 200, -1600);

    
    assert_eq!(grid.pool[10], 
        Agent{id:  201, x:-2000, y:  1600, ..Default::default()});
    assert_eq!(grid.pool[11], 
        Agent{id:  202, x: 2000, y:  1600, ..Default::default()});
    assert_eq!(grid.pool[12], 
        Agent{id:  203, x: 2000, y: -1600, ..Default::default()});
    assert_eq!(grid.pool[13], 
        Agent{id:  204, x:-2000, y: -1600, ..Default::default()});
    assert_eq!(grid.pool[14], 
        Agent{id:  205, x:-2000, y:   300, ..Default::default()});
    assert_eq!(grid.pool[15], 
        Agent{id:  206, x: 2000, y:   300, ..Default::default()});
    assert_eq!(grid.pool[16], 
        Agent{id:  207, x:  200, y:  1600, ..Default::default()});
    assert_eq!(grid.pool[17], 
        Agent{id:  208, x:  200, y: -1600, ..Default::default()});
    
}


#[test]
fn out_bounds_remove_work() {
    let mut grid = UGrid::default();
    grid.init_test_data();

    grid.insert(205, -2000, 300);
    grid.insert(206, 2000, 300);
    grid.insert(207, 200, 1600);
    grid.insert(208, 200, -1600);

    grid.remove(205, -2000, 300);
    grid.remove(208, 200, -1600);

    assert_eq!(grid.pool[10], 
        Agent{id:  INACTIVE, x:-2000, y:  300,  ..Default::default()});
    assert_eq!(grid.pool[11], 
        Agent{id:  206,      x: 2000, y:  300,  ..Default::default()});
    assert_eq!(grid.pool[12], 
        Agent{id:  207,      x:  200, y: 1600,  ..Default::default()});
    assert_eq!(grid.pool[13], 
        Agent{id:  INACTIVE, x:  200, y:-1600, next:INVALID, next_free:10});

}



#[test]
fn clear_work() {
    let mut grid = UGrid::default();

    grid.init_test_data();
    grid.clear();

    assert_eq!(grid.cols, 20);
    assert_eq!(grid.rows, 12);

    for trow in 0..grid.rows {
        for tcol in 0..grid.cols {

            assert!(grid.cells[trow][tcol].is_empty());
        }
    }

    assert_eq!(grid.pool.size, 0);
    assert_eq!(grid.pool.first_free, INVALID);
    assert_eq!(grid.pool.capacity(), 0);
}