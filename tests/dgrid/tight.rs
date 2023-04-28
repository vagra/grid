use grid::{
    *, 
    dgrid::{titem::*, tight::*}
};

#[test]
fn default_work() {
    let tight = Tight::default();

    assert_eq!(tight.pool.size, 0);
    assert_eq!(tight.cells.len(), tight.rows);
    assert_eq!(tight.cells[0].len(), tight.cols);
}

#[test]
fn insert_work() {
    let mut tight = Tight::default();

    tight.init_test_data();

    assert_eq!(tight.cells[1][1].head, 7);
    assert_eq!(tight.cells[1][3].head, 3);
    assert_eq!(tight.cells[2][1].head, 8);
    assert_eq!(tight.cells[3][8].head, 1);
    assert_eq!(tight.cells[3][11].head, 4);
    assert_eq!(tight.cells[3][12].head, 5);
    assert_eq!(tight.cells[4][8].head, 0);
    assert_eq!(tight.cells[6][3].head, 9);
    assert_eq!(tight.cells[8][6].head, 2);

    assert_eq!(tight.pool[9],
        TItem{ lcol: 13, lrow: 24, next:INVALID, next_free:INVALID }
    );
    assert_eq!(tight.pool[8],
        TItem{ lcol:  6, lrow:  8, next:INVALID, next_free:INVALID }
    );
    assert_eq!(tight.pool[7],
        TItem{ lcol:  5, lrow:  7, next:    6, next_free:INVALID }
    );
    assert_eq!(tight.pool[6],
        TItem{ lcol:  4, lrow:  6, next:INVALID, next_free:INVALID }
    );
    assert_eq!(tight.pool[1],
        TItem{ lcol: 33, lrow: 15, next:INVALID, next_free:INVALID }
    );
    assert_eq!(tight.pool[0],
        TItem{ lcol: 32, lrow: 17, next:INVALID, next_free:INVALID }
    )
}

#[test]
fn index_work() {
    let mut tight = Tight::default();

    tight.init_test_data();

    assert_eq!(tight[(1, 1)],
        TItem{ lcol:  5, lrow:  7, next:    6, next_free:INVALID }
    );
    assert_eq!(tight.pool[tight[(1, 1)].next],
        TItem{ lcol:  4, lrow:  6, next:INVALID, next_free:INVALID }
    );
}

#[test]
fn remove_lcell_work() {

    let mut tight = Tight::default();

    tight.init_test_data();

    tight.remove_lcell(5, 7);
    tight.remove_lcell(32, 17);
    tight.remove_lcell(47, 13);

    assert_eq!(tight.cells[1][1].head, 6);
    assert_eq!(tight.cells[1][3].head, 3);
    assert_eq!(tight.cells[2][1].head, 8);
    assert_eq!(tight.cells[3][8].head, 1);
    assert_eq!(tight.cells[3][12].head, 5);
    assert_eq!(tight.cells[6][3].head, 9);
    assert_eq!(tight.cells[8][6].head, 2);

    assert_eq!(tight.pool[0],
        TItem{ lcol:INVALID, lrow: 17, next:INVALID, next_free:    7 }
    );
    assert_eq!(tight.pool[4],
        TItem{ lcol:INVALID, lrow: 13, next:INVALID, next_free:    0 }
    );
    assert_eq!(tight.pool[7],
        TItem{ lcol:INVALID, lrow:  7, next:    6, next_free:INVALID }
    )

}

/*
#[test]
fn move_cell_work() {
    let mut tight = Tight::default();

    tight.init_test_data();

    tight.move_cell(107, 35, 35, 143, -165);
    tight.move_cell(106, 24, 62, 112, -123);
    
    assert_eq!(tight.cells[5][10].lhead, 9);
    assert_eq!(tight.cells[7][11].lhead, 6);

    assert_eq!(tight.pool[9],
        TItem{id:109, x:21, y:23, next:8, ..Default::default()}
    );
    assert_eq!(tight.pool[8],
        TItem{id:108, x:42, y:43, next:1, ..Default::default()}
    );
    assert_eq!(tight.pool[1],
        TItem{id:101, x:12, y:23, next:0, ..Default::default()}
    );
    assert_eq!(tight.pool[0],
        TItem{id:100, x:54, y:29, ..Default::default()}
    );

    assert_eq!(tight.pool[6],
        TItem{id:106, x:112, y:-123, next:7, ..Default::default()}
    );
    assert_eq!(tight.pool[7],
        TItem{id:107, x:143, y:-165, next:4, ..Default::default()}
    );
    assert_eq!(tight.pool[4],
        TItem{id:104, x:123, y:-123, ..Default::default()}
    );

    tight.move_cell(106, 112, -123, 24, 62);

    assert_eq!(tight.cells[5][10].lhead, 6);
    assert_eq!(tight.cells[7][11].lhead, 7);

    assert_eq!(tight.pool[6],
        TItem{id:106, x:24, y:62, next:9, ..Default::default()}
    );
    assert_eq!(tight.pool[9],
        TItem{id:109, x:21, y:23, next:8, ..Default::default()}
    );
    assert_eq!(tight.pool[8],
        TItem{id:108, x:42, y:43, next:1, ..Default::default()}
    );
    assert_eq!(tight.pool[1],
        TItem{id:101, x:12, y:23, next:0, ..Default::default()}
    );
    assert_eq!(tight.pool[0],
        TItem{id:100, x:54, y:29, ..Default::default()}
    );

    assert_eq!(tight.pool[7],
        TItem{id:107, x:143, y:-165, next:4, ..Default::default()}
    );
    assert_eq!(tight.pool[4],
        TItem{id:104, x:123, y:-123, ..Default::default()}
    );


}


#[test]
fn query_work() {
    let mut tight = Tight::default();

    tight.init_test_data();

    tight.insert(201, 38, 39);
    let vec = tight.query(38, 39, 201);

    assert_eq!(vec.len(), 4);
    assert_eq!(vec, [9u16, 8u16, 7u16, 0u16]);
}


#[test]
fn dir_query_work() {
    let mut tight = Tight::default();

    tight.init_test_data();

    tight.insert(201, 25, 45);
    let mut vec:Vec<u16>;

    vec = tight.dir_query(0, 25, 45, 201);
    assert_eq!(vec.len(), 1);
    assert_eq!(vec, [7u16]);

    vec = tight.dir_query(1, 25, 45, 201);
    assert_eq!(vec.len(), 2);
    assert_eq!(vec, [8u16, 7u16]);

    vec = tight.dir_query(2, 25, 45, 201);
    assert_eq!(vec.len(), 2);
    assert_eq!(vec, [8u16, 7u16]);

    vec = tight.dir_query(3, 25, 45, 201);
    assert_eq!(vec.len(), 0);

    vec = tight.dir_query(4, 25, 45, 201);
    assert_eq!(vec.len(), 1);
    assert_eq!(vec, [6u16]);

    vec = tight.dir_query(5, 25, 45, 201);
    assert_eq!(vec.len(), 1);
    assert_eq!(vec, [6u16]);

    vec = tight.dir_query(6, 25, 45, 201);
    assert_eq!(vec.len(), 0);

    vec = tight.dir_query(7, 25, 45, 201);
    assert_eq!(vec.len(), 0);
}
*/


#[test]
fn in_grid_work() {

    let mut tight = Tight::default();

    tight.init_test_data();

    assert!(tight.in_grid(-960, 540));
    assert!(!tight.in_grid(-961, 540));
    assert!(!tight.in_grid(-960, 541));

    assert!(tight.in_grid(959, 540));
    assert!(!tight.in_grid(960, 540));
    assert!(!tight.in_grid(959, 541));

    assert!(tight.in_grid(959, -539));
    assert!(!tight.in_grid(960, -539));
    assert!(!tight.in_grid(959, -540));

    assert!(tight.in_grid(-960, -539));
    assert!(!tight.in_grid(-961, -539));
    assert!(!tight.in_grid(-960, -540));
}

/*
#[test]
fn in_cell_work() {

    let mut tight = Tight::default();

    tight.init_test_data();

    assert!(tight.in_cell(108, 5, 10));
    assert!(tight.in_cell(106, 5, 10));
    assert!(tight.in_cell(101, 5, 10));

    assert!(tight.in_cell(104, 7, 11));
    assert!(!tight.in_cell(107, 7, 11));
    
}
*/

#[test]
fn lcell2tcell_work() {

    let tight = Tight::default();
    
    assert_eq!((8, 4), tight.lcell2tcell(32, 17));
    assert_eq!((3, 1), tight.lcell2tcell(14, 7));
    assert_eq!((1, 2), tight.lcell2tcell(6, 8));

    assert_eq!((0, 0), tight.lcell2tcell(0, 0));
    assert_eq!((15, 0), tight.lcell2tcell(63, 0));
    assert_eq!((15, 8), tight.lcell2tcell(63, 35));
    assert_eq!((0, 8), tight.lcell2tcell(0, 35));
}



#[test]
fn clear_work() {
    let mut tight = Tight::default();

    tight.init_test_data();
    tight.clear();

    assert_eq!(tight.cols, 16);
    assert_eq!(tight.rows, 9);

    for trow in 0..tight.rows {
        for tcol in 0..tight.cols {

            assert!(tight.cells[trow][tcol].is_empty());
        }
    }

    assert_eq!(tight.pool.size, 0);
    assert_eq!(tight.pool.first_free, INVALID);
    assert_eq!(tight.pool.capacity(), 0);
}