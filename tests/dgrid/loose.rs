use grid::{
    *, 
    dgrid::{agent::*, loose::*}
};

#[test]
fn default_work() {
    let loose = Loose::default();

    assert_eq!(loose.pool.size, 0);
    assert_eq!(loose.cells.len(), loose.rows);
    assert_eq!(loose.cells[0].len(), loose.cols);
}

#[test]
fn insert_work() {
    let mut loose = Loose::default();

    loose.init_test_data();

    assert_eq!(loose.cells[7][24].head, 5);
    assert_eq!(loose.cells[12][48].head, 7);
    assert_eq!(loose.cells[13][47].head, 6);
    assert_eq!(loose.cells[14][33].head, 3);
    assert_eq!(loose.cells[17][32].head, 8);
    assert_eq!(loose.cells[32][27].head, 4);

    assert_eq!(loose.pool[8],
        Agent{ id:109, x: 15, y: 27, hw: 10, hh: 10, next:2, next_free:INVALID }
    );
    assert_eq!(loose.pool[7],
        Agent{ id:108, x:480, y:170, hw: 10, hh: 10, next:INVALID, next_free:INVALID }
    );
    assert_eq!(loose.pool[6],
        Agent{ id:107, x:450, y:123, hw: 10, hh: 10, next:INVALID, next_free:INVALID }
    );
    assert_eq!(loose.pool[1],
        Agent{ id:102, x: 12, y: 10, hw: 10, hh: 10, next:0, next_free:INVALID }
    );
    assert_eq!(loose.pool[0],
        Agent{ id:101, x: 23, y: 24, hw: 10, hh: 10, next:INVALID, next_free:INVALID }
    )
}

#[test]
fn index_work() {
    let mut loose = Loose::default();

    loose.init_test_data();

    assert_eq!(loose[(17, 32)],
        Agent{ id:109, x: 15, y: 27, hw: 10, hh: 10, next:2, next_free:INVALID }
    );
    assert_eq!(loose.pool[loose[(17, 32)].next],
        Agent{ id:103, x:  6, y: 23, hw: 10, hh: 10, next:1, next_free:INVALID }
    );
    assert_eq!(loose.pool[loose.pool[loose[(17, 32)].next].next],
        Agent{ id:102, x: 12, y: 10, hw: 10, hh: 10, next:0, next_free:INVALID }
    );
    assert_eq!(loose.pool[loose.pool[loose.pool[loose[(17, 32)].next].next].next],
        Agent{ id:101, x: 23, y: 24, hw: 10, hh: 10, next:INVALID, next_free:INVALID }
    );

}

#[test]
fn remove_work() {

    let mut loose = Loose::default();

    loose.init_test_data();

    loose.remove(102, 12, 10);
    loose.remove(103, 6, 23);

    assert_eq!(loose.cells[7][24].head, 5);
    assert_eq!(loose.cells[12][48].head, 7);
    assert_eq!(loose.cells[13][47].head, 6);
    assert_eq!(loose.cells[14][33].head, 3);
    assert_eq!(loose.cells[17][32].head, 8);
    assert_eq!(loose.cells[32][27].head, 4);

    assert_eq!(loose.pool[8],
        Agent{ id:109, x: 15, y: 27, hw: 10, hh: 10, next:0, next_free:INVALID }
    );
    assert_eq!(loose.pool[2],
        Agent{ id:INACTIVE, x:  6, y: 23, hw: 10, hh: 10, next: 0, next_free: 1 }
    );
    assert_eq!(loose.pool[1],
        Agent{ id:INACTIVE, x: 12, y: 10, hw: 10, hh: 10, next: 0, next_free:INVALID }
    )

}

/*
#[test]
fn move_cell_work() {
    let mut loose = Loose::default();

    loose.init_test_data();

    loose.move_cell(107, 35, 35, 143, -165);
    loose.move_cell(106, 24, 62, 112, -123);
    
    assert_eq!(loose.cells[5][10].head, 9);
    assert_eq!(loose.cells[7][11].head, 6);

    assert_eq!(loose.pool[9],
        Agent{id:109, x:21, y:23, next:8, ..Default::default()}
    );
    assert_eq!(loose.pool[8],
        Agent{id:108, x:42, y:43, next:1, ..Default::default()}
    );
    assert_eq!(loose.pool[1],
        Agent{id:101, x:12, y:23, next:0, ..Default::default()}
    );
    assert_eq!(loose.pool[0],
        Agent{id:100, x:54, y:29, ..Default::default()}
    );

    assert_eq!(loose.pool[6],
        Agent{id:106, x:112, y:-123, next:7, ..Default::default()}
    );
    assert_eq!(loose.pool[7],
        Agent{id:107, x:143, y:-165, next:4, ..Default::default()}
    );
    assert_eq!(loose.pool[4],
        Agent{id:104, x:123, y:-123, ..Default::default()}
    );

    loose.move_cell(106, 112, -123, 24, 62);

    assert_eq!(loose.cells[5][10].head, 6);
    assert_eq!(loose.cells[7][11].head, 7);

    assert_eq!(loose.pool[6],
        Agent{id:106, x:24, y:62, next:9, ..Default::default()}
    );
    assert_eq!(loose.pool[9],
        Agent{id:109, x:21, y:23, next:8, ..Default::default()}
    );
    assert_eq!(loose.pool[8],
        Agent{id:108, x:42, y:43, next:1, ..Default::default()}
    );
    assert_eq!(loose.pool[1],
        Agent{id:101, x:12, y:23, next:0, ..Default::default()}
    );
    assert_eq!(loose.pool[0],
        Agent{id:100, x:54, y:29, ..Default::default()}
    );

    assert_eq!(loose.pool[7],
        Agent{id:107, x:143, y:-165, next:4, ..Default::default()}
    );
    assert_eq!(loose.pool[4],
        Agent{id:104, x:123, y:-123, ..Default::default()}
    );


}


#[test]
fn query_work() {
    let mut loose = Loose::default();

    loose.init_test_data();

    loose.insert(201, 38, 39);
    let vec = loose.query(38, 39, 201);

    assert_eq!(vec.len(), 4);
    assert_eq!(vec, [9u16, 8u16, 7u16, 0u16]);
}


#[test]
fn dir_query_work() {
    let mut loose = Loose::default();

    loose.init_test_data();

    loose.insert(201, 25, 45);
    let mut vec:Vec<u16>;

    vec = loose.dir_query(0, 25, 45, 201);
    assert_eq!(vec.len(), 1);
    assert_eq!(vec, [7u16]);

    vec = loose.dir_query(1, 25, 45, 201);
    assert_eq!(vec.len(), 2);
    assert_eq!(vec, [8u16, 7u16]);

    vec = loose.dir_query(2, 25, 45, 201);
    assert_eq!(vec.len(), 2);
    assert_eq!(vec, [8u16, 7u16]);

    vec = loose.dir_query(3, 25, 45, 201);
    assert_eq!(vec.len(), 0);

    vec = loose.dir_query(4, 25, 45, 201);
    assert_eq!(vec.len(), 1);
    assert_eq!(vec, [6u16]);

    vec = loose.dir_query(5, 25, 45, 201);
    assert_eq!(vec.len(), 1);
    assert_eq!(vec, [6u16]);

    vec = loose.dir_query(6, 25, 45, 201);
    assert_eq!(vec.len(), 0);

    vec = loose.dir_query(7, 25, 45, 201);
    assert_eq!(vec.len(), 0);
}
*/


#[test]
fn in_grid_work() {

    let mut loose = Loose::default();

    loose.init_test_data();

    assert!(loose.in_grid(-960, 540));
    assert!(!loose.in_grid(-961, 540));
    assert!(!loose.in_grid(-960, 541));

    assert!(loose.in_grid(959, 540));
    assert!(!loose.in_grid(960, 540));
    assert!(!loose.in_grid(959, 541));

    assert!(loose.in_grid(959, -539));
    assert!(!loose.in_grid(960, -539));
    assert!(!loose.in_grid(959, -540));

    assert!(loose.in_grid(-960, -539));
    assert!(!loose.in_grid(-961, -539));
    assert!(!loose.in_grid(-960, -540));
}

/*
#[test]
fn in_cell_work() {

    let mut loose = Loose::default();

    loose.init_test_data();

    assert!(loose.in_cell(108, 5, 10));
    assert!(loose.in_cell(106, 5, 10));
    assert!(loose.in_cell(101, 5, 10));

    assert!(loose.in_cell(104, 7, 11));
    assert!(!loose.in_cell(107, 7, 11));
    
}
*/

#[test]
fn pos2lcell_work() {

    let loose = Loose::default();
    
    assert_eq!((0, 0), loose.pos2lcell(-2000, 1600));
    assert_eq!((63, 0), loose.pos2lcell(2000, 1600));
    assert_eq!((63, 35), loose.pos2lcell(2000, -1600));
    assert_eq!((0, 35), loose.pos2lcell(-2000, -1600));

    assert_eq!((0, 8), loose.pos2lcell(-2000, 300));
    assert_eq!((63, 8), loose.pos2lcell(2000, 300));
    assert_eq!((38, 0), loose.pos2lcell(200, 1600));
    assert_eq!((38, 35), loose.pos2lcell(200, -1600));
}

#[test]
fn out_bounds_insert_work() {
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

    assert_eq!(loose[(0, 0)], 
        Agent{ id:201, x:-2000, y:1600, hw: 10, hh: 10, next:INVALID, next_free:INVALID }
    );
    assert_eq!(loose[(0, 63)], 
        Agent{ id:202, x:2000, y:1600, hw: 10, hh: 10, next:INVALID, next_free:INVALID }
    );
    assert_eq!(loose[(35, 63)], 
        Agent{ id:203, x:2000, y:-1600, hw: 10, hh: 10, next:INVALID, next_free:INVALID }
    );
    assert_eq!(loose[(35, 0)], 
        Agent{ id:204, x:-2000, y:-1600, hw: 10, hh: 10, next:INVALID, next_free:INVALID }
    );
    assert_eq!(loose[(8, 0)], 
        Agent{ id:205, x:-2000, y:300, hw: 10, hh: 10, next:INVALID, next_free:INVALID }
    );
    assert_eq!(loose[(8, 63)], 
        Agent{ id:206, x:2000, y:300, hw: 10, hh: 10, next:INVALID, next_free:INVALID }
    );
    assert_eq!(loose[(0, 38)], 
        Agent{ id:207, x:200, y:1600, hw: 10, hh: 10, next:INVALID, next_free:INVALID }
    );
    assert_eq!(loose[(35, 38)], 
        Agent{ id:208, x:200, y:-1600, hw: 10, hh: 10, next:INVALID, next_free:INVALID }
    );
}


#[test]
fn out_bounds_remove_work() {
    let mut loose = Loose::default();
    loose.init_test_data();

    loose.insert(205, -2000, 300, 10, 10);
    loose.insert(206, 2000, 300, 10, 10);
    loose.insert(207, 200, 1600, 10, 10);
    loose.insert(208, 200, -1600, 10, 10);

    loose.remove(205, -2000, 300);
    loose.remove(208, 200, -1600);

    assert_eq!(loose.pool[9], 
        Agent{ id:INACTIVE, x:-2000, y:300, hw: 10, hh: 10, next:INVALID, next_free:INVALID }
    );
    assert_eq!(loose.pool[10], 
        Agent{ id:206, x:2000, y:300, hw: 10, hh: 10, next:INVALID, next_free:INVALID }
    );
    assert_eq!(loose.pool[11], 
        Agent{ id:207, x:200, y:1600, hw: 10, hh: 10, next:INVALID, next_free:INVALID }
    );
    assert_eq!(loose.pool[12], 
        Agent{ id:INACTIVE, x:200, y:-1600, hw: 10, hh: 10, next:INVALID, next_free:9 }
    );
}



#[test]
fn clear_work() {
    let mut loose = Loose::default();

    loose.init_test_data();
    loose.clear();

    assert_eq!(loose.cols, 64);
    assert_eq!(loose.rows, 36);

    for lrow in 0..loose.rows {
        for lcol in 0..loose.cols {

            assert!(loose.cells[lrow][lcol].is_empty());
        }
    }

    assert_eq!(loose.pool.size, 0);
    assert_eq!(loose.pool.first_free, INVALID);
    assert_eq!(loose.pool.capacity(), 0);
}