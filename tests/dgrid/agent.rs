use grid::{
    *, 
    dgrid::{*, agent::*},
};



#[test]
fn is_free_disable_work() {
    let mut agent = Agent::new(101, 30, 40, 10, 10);
    assert!(!agent.is_free());

    agent.disable();
    assert!(agent.is_free());
}

#[test]
fn in_grid_work() {

    let grid = DGrid::default();

    let a = Agent::new(102, -960, 540, 10, 10);
    assert!(a.in_grid(&grid));
    let b = Agent::new(102, 959, 540, 10, 10);
    assert!(b.in_grid(&grid));
    let c = Agent::new(102, 959, -539, 10, 10);
    assert!(c.in_grid(&grid));
    let d = Agent::new(102, -960, -539, 10, 10);
    assert!(d.in_grid(&grid));

    let e = Agent::new(102, -961, 540, 10, 10);
    assert!(!e.in_grid(&grid));
    let f = Agent::new(102, -960, 541, 10, 10);
    assert!(!f.in_grid(&grid));

    let g = Agent::new(102, -961, 540, 10, 10);
    assert!(!g.in_grid(&grid));
    let h = Agent::new(102, -960, 541, 10, 10);
    assert!(!h.in_grid(&grid));

    let i = Agent::new(102, 960, -539, 10, 10);
    assert!(!i.in_grid(&grid));
    let j = Agent::new(102, 959, -540, 10, 10);
    assert!(!j.in_grid(&grid));

    let l = Agent::new(102, -961, -539, 10, 10);
    assert!(!l.in_grid(&grid));
    let m = Agent::new(102, -960, -540, 10, 10);
    assert!(!m.in_grid(&grid));

}