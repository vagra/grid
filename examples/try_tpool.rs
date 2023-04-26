#![allow(dead_code)]
use grid::{
    pool::*,
    dgrid::titem::*,
};


fn main() {
    test_insert_erase_clear();
}

fn test_insert_erase_clear() {
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
    
    tpool.print();

    tpool.erase(2);
    println!("erase 2");
    tpool.print();

    tpool.erase(0);
    println!("erase 0");
    tpool.print();

    tpool.erase(5);
    println!("erase 5");
    tpool.print();

    let mut index = tpool.insert(TItem::new(20, 20));
    println!("insert {}", index);
    tpool.print();

    index = tpool.insert(TItem::new(21, 21));
    println!("insert {}", index);
    tpool.print();

    index = tpool.insert(TItem::new(22, 22));
    println!("insert {}", index);
    tpool.print();

    index = tpool.insert(TItem::new(23, 23));
    println!("insert {}", index);
    tpool.print();

    println!("clear");
    tpool.clear();
    tpool.print();

}