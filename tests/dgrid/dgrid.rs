use grid::{*, dgrid::{*, agent::*, titem::*, rect::*}};



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




#[test]
fn move_work() {

    let mut grid = DGrid::default();
    grid.init_test_data();

    grid.move_cell(107, 450, 123, 470, 150);

    assert_eq!(grid.loose.pool[6],
        Agent{ id:107, x: 470, y: 150, hw: 10, hh: 10, next:INVALID, next_free:INVALID }
    );

    assert_eq!(grid.loose.cells[13][47].head, 6);
    assert_eq!(grid.loose.cells[13][47].rect, 
        LRect{l:440, t:160, r:480, b:113}
    );

    assert_eq!(grid.tight.cells[3][12].head, 10);
    assert_eq!(grid.tight.pool[10],
        TItem{ lcol: 47, lrow: 13, next:    9, next_free:INVALID }
    );

}



#[test]
fn optimize_work() {

    let mut grid = DGrid::default();
    grid.init_test_data();

    grid.remove(103, 6, 23);
    grid.remove(106, -234, 324);
    grid.remove(109, 15, 27);

    grid.move_cell(107, 450, 123, 470, 150);

    grid.optimize();

    assert_eq!(grid.loose.pool.first_free, INVALID);
    assert_eq!(grid.loose.pool.capacity(), 6);

    assert_eq!(grid.loose.cells[12][48].head, 0);
    assert_eq!(grid.loose.cells[13][47].head, 1);
    assert_eq!(grid.loose.cells[14][33].head, 2);
    assert_eq!(grid.loose.cells[17][32].head, 4);
    assert_eq!(grid.loose.cells[32][27].head, 5);

    assert_eq!(grid.loose.pool[0],
        Agent{ id:108, x: 480, y: 170, hw: 10, hh: 10, next:INVALID, next_free:INVALID }
    );
    assert_eq!(grid.loose.pool[1],
        Agent{ id:107, x: 470, y: 150, hw: 10, hh: 10, next:INVALID, next_free:INVALID }
    );
    assert_eq!(grid.loose.pool[2],
        Agent{ id:104, x:  40, y:  97, hw: 10, hh: 10, next:INVALID, next_free:INVALID }
    );
    assert_eq!(grid.loose.pool[3],
        Agent{ id:102, x:  12, y:  10, hw: 10, hh: 10, next:INVALID, next_free:INVALID }
    );
    assert_eq!(grid.loose.pool[4],
        Agent{ id:101, x:  23, y:  24, hw: 10, hh: 10, next:    3, next_free:INVALID }
    );
    assert_eq!(grid.loose.pool[5],
        Agent{ id:105, x:-123, y:-432, hw: 10, hh: 10, next:INVALID, next_free:INVALID }
    );


    assert_eq!(grid.tight.pool.first_free, INVALID);
    assert_eq!(grid.tight.pool.capacity(), 8);

    assert_eq!(grid.tight.cells[3][8].head, 4);
    assert_eq!(grid.tight.cells[3][11].head, 2);
    assert_eq!(grid.tight.cells[3][12].head, 3);
    assert_eq!(grid.tight.cells[4][8].head, 5);
    assert_eq!(grid.tight.cells[8][6].head, 6);
    assert_eq!(grid.tight.cells[8][7].head, 7);

    assert_eq!(grid.tight.pool[0],
        TItem{ lcol: 48, lrow: 12, next:INVALID, next_free:INVALID }
    );
    assert_eq!(grid.tight.pool[1],
        TItem{ lcol: 48, lrow: 12, next:INVALID, next_free:INVALID }
    );
    assert_eq!(grid.tight.pool[2],
        TItem{ lcol: 47, lrow: 13, next:    0, next_free:INVALID }
    );
    assert_eq!(grid.tight.pool[3],
        TItem{ lcol: 47, lrow: 13, next:    1, next_free:INVALID }
    );
    assert_eq!(grid.tight.pool[4],
        TItem{ lcol: 33, lrow: 14, next:INVALID, next_free:INVALID }
    );
    assert_eq!(grid.tight.pool[5],
        TItem{ lcol: 32, lrow: 17, next:INVALID, next_free:INVALID }
    );
    assert_eq!(grid.tight.pool[6],
        TItem{ lcol: 27, lrow: 32, next:INVALID, next_free:INVALID }
    );
    assert_eq!(grid.tight.pool[7],
        TItem{ lcol: 27, lrow: 32, next:INVALID, next_free:INVALID }
    );

}