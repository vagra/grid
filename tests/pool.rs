mod pool {

    use grid::{agent::*, pool::*};


    #[test]
    fn basics() {
        let mut pool = Pool::default();

        pool.insert(Agent::new(1, 10, 10));
        pool.insert(Agent::new(2, 20, 20));
        pool.insert(Agent::new(3, 30, 30));

        assert_eq!(pool.data[2], 
            Agent{id: 3, x:30, y:30, ..Default::default()});
        assert_eq!(pool.data[1],
            Agent{id: 2, x:20, y:20, ..Default::default()});
        assert_eq!(pool.data[0],
            Agent{id: 1, x:10, y:10, ..Default::default()});
        
        assert_eq!(pool.size, 3);
        assert_eq!(pool.capacity(), 3);
        assert_eq!(pool.first_free, INVALID);
    }


    #[test]
    fn erase_insert() {
        let mut pool = Pool::default();

        pool.insert(Agent::new(100, 10, 10));
        pool.insert(Agent::new(101, 20, 20));
        pool.insert(Agent::new(102, 30, 30));
        pool.insert(Agent::new(103, 40, 40));
        pool.insert(Agent::new(104, 50, 50));
        pool.insert(Agent::new(105, 60, 60));
        pool.insert(Agent::new(106, 70, 70));
        pool.insert(Agent::new(107, 80, 80));
        pool.insert(Agent::new(108, 90, 90));


        pool.erase(2);
        assert_eq!(pool.data[2], 
            Agent{id: INACTIVE, x: 30, y:30, ..Default::default()}
        );
        assert_eq!(pool.first_free, 2);
        assert_eq!(pool.size, 8);
        assert_eq!(pool.capacity(), 9);

        pool.erase(0);
        assert_eq!(pool.data[0], 
            Agent{id: INACTIVE, x: 10, y:10, next_free: 2, ..Default::default()}
        );
        assert_eq!(pool.first_free, 0);
        assert_eq!(pool.size, 7);
        assert_eq!(pool.capacity(), 9);

        pool.erase(5);
        assert_eq!(pool.data[5], 
            Agent{id: INACTIVE, x: 60, y:60, next_free: 0, ..Default::default()}
        );
        assert_eq!(pool.first_free, 5);
        assert_eq!(pool.size, 6);
        assert_eq!(pool.capacity(), 9);

        let mut index = pool.insert(Agent::new(110, 10, 10));

        assert_eq!(pool.data[5], 
            Agent{id: 110, x: 10, y:10, ..Default::default()}
        );
        assert_eq!(index, 5);
        assert_eq!(pool.first_free, 0);
        assert_eq!(pool.size, 7);
        assert_eq!(pool.capacity(), 9);
        assert_eq!(pool.data[pool.first_free].next_free, 2);

        index = pool.insert(Agent::new(111, 11, 11));

        assert_eq!(pool.data[0], 
            Agent{id: 111, x: 11, y:11, ..Default::default()}
        );
        assert_eq!(index, 0);
        assert_eq!(pool.first_free, 2);
        assert_eq!(pool.size, 8);
        assert_eq!(pool.capacity(), 9);
        assert_eq!(pool.data[pool.first_free].next_free, INVALID);

        index = pool.insert(Agent::new(112, 12, 12));

        assert_eq!(pool.data[2], 
            Agent{id: 112, x: 12, y:12, ..Default::default()}
        );
        assert_eq!(index, 2);
        assert_eq!(pool.first_free, INVALID);
        assert_eq!(pool.size, 9);
        assert_eq!(pool.capacity(), 9);

        index = pool.insert(Agent::new(115, 15, 15));

        assert_eq!(pool.data[9], 
            Agent{id: 115, x: 15, y:15, ..Default::default()}
        );
        assert_eq!(index, 9);
        assert_eq!(pool.first_free, INVALID);
        assert_eq!(pool.size, 10);
        assert_eq!(pool.capacity(), 10);
    }


    #[test]
    fn clear() {
        let mut pool = Pool::default();

        pool.insert(Agent::new(100, 10, 10));
        pool.insert(Agent::new(101, 20, 20));
        pool.insert(Agent::new(102, 30, 30));

        assert_eq!(pool.first_free, INVALID);
        assert_eq!(pool.size, 3);
        assert_eq!(pool.capacity(), 3);

        pool.clear();
        assert_eq!(pool.first_free, INVALID);
        assert_eq!(pool.size, 0);
        assert_eq!(pool.capacity(), 0);

        pool.clear();
        assert_eq!(pool.first_free, INVALID);
        assert_eq!(pool.size, 0);
        assert_eq!(pool.capacity(), 0);
    }


    #[test]
    fn index() {

        let mut pool = Pool::default();

        pool.insert(Agent::new(100, 10, 10));
        pool.insert(Agent::new(101, 20, 20));
        pool.insert(Agent::new(102, 30, 30));
        pool.insert(Agent::new(103, 40, 40));
        pool.insert(Agent::new(104, 50, 50));

        assert_eq!(pool[3],
            Agent{id: 103, x: 40, y:40, ..Default::default()}
        );

        pool[2].x = 35;
        pool[2].y = 45;
        assert_eq!(pool[2],
            Agent{id: 102, x: 35, y:45, ..Default::default()}
        );

    }


    #[test]
    fn after_construction_has_no_first_free() {
        let pool = Pool::default();
        assert_eq!(pool.first_free, INVALID);
        assert_eq!(pool.capacity(), 0);
    }

    #[test]
    fn after_insertion_has_no_first_free() {
        let mut pool = Pool::default();
        assert_eq!(pool.insert(Agent::new(1, 10, 10)), 0);
        assert_eq!(pool.first_free, INVALID);
        assert_eq!(pool.capacity(), 1);
    }

    #[test]
    fn after_deletion_has_first_free() {
        let mut pool = Pool::default();
        pool.insert(Agent::new(1, 10, 10));
        pool.erase(0);
        assert_eq!(pool.first_free, 0);
        assert_eq!(pool.capacity(), 1);
    }

    #[test]
    fn insert_after_delete_has_no_first_free() {
        let mut pool = Pool::default();
        pool.insert(Agent::new(1, 10, 10));
        pool.erase(0);
        pool.insert(Agent::new(2, 20, 20));
        assert_eq!(pool.first_free, INVALID);
        assert_eq!(pool.capacity(), 1);
    }

    #[test]
    fn first_free_points_to_last_erased_index() {
        let mut pool = Pool::default();
        insert_some(&mut pool, 2);
        pool.erase(0);
        pool.erase(1);
        assert_eq!(pool.first_free, 1);
        assert_eq!(pool.capacity(), 2);
    }

    #[test]
    fn erase_in_ascending_order() {
        let mut pool = Pool::default();
        insert_some(&mut pool, 4);
        pool.erase(0);
        pool.erase(1);
        pool.erase(2);
        pool.erase(3);
        assert_eq!(pool.first_free, 3);
        assert_eq!(pool.capacity(), 4);
    }

    #[test]
    fn erase_in_descending_order() {
        let mut pool = Pool::default();
        insert_some(&mut pool, 4);
        pool.erase(3);
        pool.erase(2);
        pool.erase(1);
        pool.erase(0);
        assert_eq!(pool.first_free, 0);
        assert_eq!(pool.capacity(), 4);
    }

    #[test]
    fn erase_in_mixed_order() {
        let mut pool = Pool::default();
        insert_some(&mut pool, 4);
        pool.erase(0);
        pool.erase(3);
        pool.erase(1);
        pool.erase(2);
        assert_eq!(pool.first_free, 2);
        assert_eq!(pool.capacity(), 4);
    }


    #[test]
    fn clear_works() {
        let mut pool = Pool::default();
        insert_some(&mut pool, 4);
        pool.erase(1);
        pool.clear();
        pool.clear();
        assert_eq!(pool.first_free, INVALID);
        assert_eq!(pool.capacity(), 0);
    }



    #[test]
    fn index_works() {
        let mut pool = Pool::default();
        pool.insert(Agent::new(1, 10, 10));
        pool.insert(Agent::new(2, 20, 20));

        let element = pool[0];
        assert_eq!(element, Agent::new(1, 10, 10));
    }

    #[test]
    fn index_mut_works() {
        let mut pool = Pool::default();
        pool.insert(Agent::new(1, 10, 10));
        pool.insert(Agent::new(2, 20, 20));

        let mut element = &mut pool[0];
        *element = Agent::new(3, 30, 30);

        element = &mut pool[0];
        assert_eq!(*element, Agent::new(3, 30, 30));
    }


    fn insert_some(pool: &mut Pool, n: u16) {
        for i in 0..n {
            pool.insert(Agent::new(i as u32, i as i16 * 10, i as i16 * 20));
        }
    }

}