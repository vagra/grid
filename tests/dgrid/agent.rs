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

    let a = Agent::new(102, -970, 550, 10, 10);
    let b = Agent::new(102, 969, 550, 10, 10);
    let c = Agent::new(102, 969, -549, 10, 10);
    let d = Agent::new(102, -970, -549, 10, 10);

    let e = Agent::new(102, -971, 550, 10, 10);
    let f = Agent::new(102, -970, 551, 10, 10);

    let g = Agent::new(102, -971, 550, 10, 10);
    let h = Agent::new(102, -970, 551, 10, 10);

    let i = Agent::new(102, 970, -549, 10, 10);
    let j = Agent::new(102, 969, -550, 10, 10);

    let l = Agent::new(102, -971, -559, 10, 10);
    let m = Agent::new(102, -970, -550, 10, 10);

    assert!(a.in_grid(&grid));
    assert!(b.in_grid(&grid));
    assert!(c.in_grid(&grid));
    assert!(d.in_grid(&grid));

    assert!(!e.in_grid(&grid));
    assert!(!f.in_grid(&grid));
    assert!(!g.in_grid(&grid));
    assert!(!h.in_grid(&grid));
    assert!(!i.in_grid(&grid));
    assert!(!j.in_grid(&grid));
    assert!(!l.in_grid(&grid));
    assert!(!m.in_grid(&grid));

}