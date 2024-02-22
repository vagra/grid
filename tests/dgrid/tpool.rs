use grid::{
    *, pool::*, 
    dgrid::titem::*,
};


#[test]
fn insert_erase_works() {
    let mut tpool:Pool<TItem> = Pool::default();

    tpool.insert(TItem::new(1, 1));
    tpool.insert(TItem::new(2, 2));
    tpool.insert(TItem::new(3, 3));
    tpool.insert(TItem::new(4, 4));
    tpool.insert(TItem::new(5, 5));
    tpool.insert(TItem::new(6, 6));
    tpool.insert(TItem::new(7, 7));
    tpool.insert(TItem::new(8, 8));
    tpool.insert(TItem::new(9, 9));

    assert_eq!(tpool.size, 9);
    assert_eq!(tpool.first_free, INVALID);
    assert_eq!(tpool.capacity(), 9);

    tpool.erase(2);
    assert_eq!(tpool.size, 8);
    assert_eq!(tpool.first_free, 2);
    assert_eq!(tpool.capacity(), 9);
    assert_eq!(tpool.data[2].lcol, INVALID);

    tpool.erase(0);
    assert_eq!(tpool.size, 7);
    assert_eq!(tpool.first_free, 0);
    assert_eq!(tpool.capacity(), 9);
    assert_eq!(tpool.data[2].lcol, INVALID);
    assert_eq!(tpool.data[0].lcol, INVALID);
    assert_eq!(tpool.data[0].next_free, 2);

    tpool.erase(5);
    assert_eq!(tpool.size, 6);
    assert_eq!(tpool.first_free, 5);
    assert_eq!(tpool.capacity(), 9);
    assert_eq!(tpool.data[2].lcol, INVALID);
    assert_eq!(tpool.data[0].lcol, INVALID);
    assert_eq!(tpool.data[0].next_free, 2);
    assert_eq!(tpool.data[5].lcol, INVALID);
    assert_eq!(tpool.data[5].next_free, 0);

    let mut index = tpool.insert(TItem::new(20, 20));
    assert_eq!(index, 5);
    assert_eq!(tpool.size, 7);
    assert_eq!(tpool.first_free, 0);
    assert_eq!(tpool.capacity(), 9);
    assert_eq!(tpool.data[2].lcol, INVALID);
    assert_eq!(tpool.data[0].lcol, INVALID);
    assert_eq!(tpool.data[0].next_free, 2);
    assert_eq!(tpool.data[5],
        TItem{ lcol:20, lrow:20, next: INVALID, next_free: INVALID}
    );

    index = tpool.insert(TItem::new(21, 21));
    assert_eq!(index, 0);
    assert_eq!(tpool.size, 8);
    assert_eq!(tpool.first_free, 2);
    assert_eq!(tpool.capacity(), 9);
    assert_eq!(tpool.data[2].lcol, INVALID);
    assert_eq!(tpool.data[5],
        TItem{ lcol:20, lrow:20, next: INVALID, next_free: INVALID}
    );
    assert_eq!(tpool.data[0],
        TItem{ lcol:21, lrow:21, next: INVALID, next_free: INVALID}
    );

    index = tpool.insert(TItem::new(22, 22));
    assert_eq!(index, 2);
    assert_eq!(tpool.size, 9);
    assert_eq!(tpool.first_free, INVALID);
    assert_eq!(tpool.capacity(), 9);
    assert_eq!(tpool.data[5],
        TItem{ lcol:20, lrow:20, next: INVALID, next_free: INVALID}
    );
    assert_eq!(tpool.data[0],
        TItem{ lcol:21, lrow:21, next: INVALID, next_free: INVALID}
    );
    assert_eq!(tpool.data[2],
        TItem{ lcol:22, lrow:22, next: INVALID, next_free: INVALID}
    );

    index = tpool.insert(TItem::new(23, 23));
    assert_eq!(index, 9);
    assert_eq!(tpool.size, 10);
    assert_eq!(tpool.first_free, INVALID);
    assert_eq!(tpool.capacity(), 10);
    assert_eq!(tpool.data[9],
        TItem{ lcol:23, lrow:23, next: INVALID, next_free: INVALID}
    );
}


#[test]
fn clear_works() {
    let mut tpool:Pool<TItem> = Pool::default();

    insert_some(&mut tpool, 10);
    assert_eq!(tpool.size, 10);
    assert_eq!(tpool.first_free, INVALID);
    assert_eq!(tpool.capacity(), 10);

    tpool.clear();
    assert_eq!(tpool.size, 0);
    assert_eq!(tpool.first_free, INVALID);
    assert_eq!(tpool.capacity(), 0);
}


#[test]
fn index_works() {
    let mut tpool:Pool<TItem> = Pool::default();
    tpool.insert(TItem::new(10, 10));
    tpool.insert(TItem::new(20, 20));

    let mut element = tpool[0];
    assert_eq!(element,
        TItem{lcol:10, lrow:10, next:INVALID, next_free:INVALID}
    );

    element = tpool[1];
    assert_eq!(element,
        TItem{lcol:20, lrow:20, next:INVALID, next_free:INVALID}
    );
}


#[test]
fn index_mut_works() {
    let mut tpool:Pool<TItem> = Pool::default();
    tpool.insert(TItem::new(10, 10));
    tpool.insert(TItem::new(20, 20));

    let element = &mut tpool[0];
    assert_eq!(*element,
        TItem{lcol:10, lrow:10, next:INVALID, next_free:INVALID}
    );

    element.lcol = 5;
    element.lrow = 6;
    assert_eq!(tpool[0],
        TItem{lcol:5, lrow:6, next:INVALID, next_free:INVALID}
    );
}



#[test]
fn after_construction_has_no_first_free() {
    let tpool:Pool<TItem> = Pool::default();
    assert_eq!(tpool.first_free, INVALID);
    assert_eq!(tpool.capacity(), 0);
}

#[test]
fn after_insertion_has_no_first_free() {
    let mut tpool:Pool<TItem> = Pool::default();
    let index = tpool.insert(TItem::new(10, 10)); 
    assert_eq!(index, 0);
    assert_eq!(tpool.first_free, INVALID);
    assert_eq!(tpool.capacity(), 1);
}

#[test]
fn after_deletion_has_first_free() {
    let mut tpool:Pool<TItem> = Pool::default();
    let index = tpool.insert(TItem::new(10, 10));
    assert_eq!(index, 0);

    tpool.erase(0);
    assert_eq!(tpool.first_free, 0);
    assert_eq!(tpool.capacity(), 1);
}

#[test]
fn insert_after_delete_has_no_first_free() {
    let mut tpool:Pool<TItem> = Pool::default();
    let index = tpool.insert(TItem::new(10, 10));
    assert_eq!(index, 0);

    tpool.erase(0);
    tpool.insert(TItem::new(20, 20));

    assert_eq!(tpool.first_free, INVALID);
    assert_eq!(tpool.capacity(), 1);
}

#[test]
fn first_free_points_to_last_erased_index() {
    let mut tpool:Pool<TItem> = Pool::default();
    insert_some(&mut tpool, 4);
    
    tpool.erase(0);
    tpool.erase(1);
    assert_eq!(tpool.first_free, 1);
    assert_eq!(tpool.capacity(), 4);
}

#[test]
fn erase_in_ascending_order() {
    let mut tpool:Pool<TItem> = Pool::default();
    insert_some(&mut tpool, 4);
    tpool.erase(0);
    tpool.erase(1);
    tpool.erase(2);
    tpool.erase(3);
    assert_eq!(tpool.first_free, 3);
    assert_eq!(tpool.capacity(), 4);
}

#[test]
fn erase_in_descending_order() {
    let mut tpool:Pool<TItem> = Pool::default();
    insert_some(&mut tpool, 4);
    tpool.erase(3);
    tpool.erase(2);
    tpool.erase(1);
    tpool.erase(0);
    assert_eq!(tpool.first_free, 0);
    assert_eq!(tpool.capacity(), 4);
}

#[test]
fn erase_in_mixed_order() {
    let mut tpool:Pool<TItem> = Pool::default();
    insert_some(&mut tpool, 4);
    tpool.erase(0);
    tpool.erase(3);
    tpool.erase(1);
    tpool.erase(2);
    assert_eq!(tpool.first_free, 2);
    assert_eq!(tpool.capacity(), 4);
}


fn insert_some(tpool: &mut Pool<TItem>, n: u16) {
    for i in 0..n {
        tpool.insert(TItem::new(i as u16, i as u16));
    }
}