use grid::{*, dgrid::{*, agent::*, titem::*}};



#[test]
fn new_work() {
    let grid = DGrid::new(2, 20, 5, 3);

    assert_eq!(grid.width, 800);
    assert_eq!(grid.height, 480);

    assert_eq!(grid.tight.width, 800);
    assert_eq!(grid.tight.height, 480);
    assert_eq!(grid.tight.cols, 10);
    assert_eq!(grid.tight.rows, 6);
    assert_eq!(grid.tight.cell_size, 80);
    assert_eq!(grid.tight.pool.size, 0);

    assert_eq!(grid.loose.width, 800);
    assert_eq!(grid.loose.height, 480);
    assert_eq!(grid.loose.cols, 20);
    assert_eq!(grid.loose.rows, 12);
    assert_eq!(grid.loose.cell_size, 40);
    assert_eq!(grid.loose.pool.size, 0);
}

#[test]
fn insert_work() {

    let mut grid = DGrid::default();
    grid.init_test_data();

    assert_eq!(grid.loose.pool[0],
        Agent{ id:101, x: 23, y: 24, hw: 10, hh: 10, next:INVALID, next_free:INVALID }
    );
    assert_eq!(grid.loose.pool[1],
        Agent{ id:102, x: 12, y: 10, hw: 10, hh: 10, next:    0, next_free:INVALID }
    );
    assert_eq!(grid.loose.pool[2],
        Agent{ id:103, x:  6, y: 23, hw: 10, hh: 10, next:    1, next_free:INVALID }
    );
    assert_eq!(grid.loose.pool[8],
        Agent{ id:109, x: 15, y: 27, hw: 10, hh: 10, next:    2, next_free:INVALID }
    );

    assert_eq!(grid.tight.pool[0],
        TItem{ lcol: 32, lrow: 17, next:INVALID, next_free:INVALID }
    );
    assert_eq!(grid.tight.pool[1],
        TItem{ lcol: 32, lrow: 17, next:INVALID, next_free:INVALID }
    );
    assert_eq!(grid.tight.pool[8],
        TItem{ lcol: 48, lrow: 12, next:    7, next_free:INVALID }
    );
    assert_eq!(grid.tight.pool[9],
        TItem{ lcol: 48, lrow: 12, next:INVALID, next_free:INVALID }
    );
}


#[test]
fn remove_work() {

    let mut grid = DGrid::default();
    grid.init_test_data();

    grid.remove(103, 6, 23);
    grid.remove(106, -234, 324);
    grid.remove(109, 15, 27);

    assert_eq!(grid.loose.pool[2],
        Agent{ id:INACTIVE, x:  6, y: 23, hw: 10, hh: 10, next:    1, next_free:INVALID }
    );
    assert_eq!(grid.loose.pool[5],
        Agent{ id:INACTIVE, x:-234, y:324, hw: 10, hh: 10, next:INVALID, next_free:    2 }
    );
    assert_eq!(grid.loose.pool[8],
        Agent{ id:INACTIVE, x: 15, y: 27, hw: 10, hh: 10, next:    1, next_free:    5 }
    );

    assert_eq!(grid.tight.pool[0],
        TItem{ lcol: 32, lrow: 17, next:INVALID, next_free:INVALID }
    );
    assert_eq!(grid.tight.pool[1],
        TItem{ lcol: 32, lrow: 17, next:INVALID, next_free:INVALID }
    );
    assert_eq!(grid.tight.pool[8],
        TItem{ lcol: 48, lrow: 12, next:    7, next_free:INVALID }
    );
    assert_eq!(grid.tight.pool[9],
        TItem{ lcol: 48, lrow: 12, next:INVALID, next_free:INVALID }
    );

}