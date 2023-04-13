mod agent {

    use grid::{agent::*};

#[test]
fn default_work() {
    let agent = Agent::default();

    assert_eq!(agent,
        Agent {
            id: INACTIVE,
            x: 0,
            y: 0,
            ..Default::default()
        }
    )
}

#[test]
fn new_work() {
    let agent = Agent::new(1, 20, 20);

    assert_eq!(agent,
        Agent {
            id: 1,
            x: 20,
            y: 20,
            ..Default::default()
        }
    )
}

#[test]
fn disable_work() {
    let mut agent = Agent::new(1, 10, 10);

    agent.disable();

    assert_eq!(agent,
        Agent {
            id: INACTIVE,
            x: 10,
            y: 10,
            ..Default::default()
        }
    )
}

#[test]
fn is_free_work() {
    let mut agent = Agent::new(1, 10, 10);
    agent.disable();

    assert!(agent.is_free());
}

#[test]
fn at_front_work() {
    let agent = Agent::new(9, 4, -4);
    let agent0 = Agent::new(0, 2, -12);
    let agent1 = Agent::new(1, 8, -12);
    let agent2 = Agent::new(2, 12, -8);
    let agent3 = Agent::new(3, 10, 0);
    let agent4 = Agent::new(4, 2, 6);
    let agent5 = Agent::new(5, -4, 0);
    let agent6 = Agent::new(6, -4, -6);
    let agent7 = Agent::new(7, 0, -8);

    assert!(!agent0.at_front(&agent, 6));
    assert!(agent0.at_front(&agent, 7));
    assert!(agent0.at_front(&agent, 0));
    assert!(!agent0.at_front(&agent, 1));

    assert!(!agent1.at_front(&agent, 7));
    assert!(agent1.at_front(&agent, 0));
    assert!(agent1.at_front(&agent, 1));
    assert!(!agent1.at_front(&agent, 2));

    assert!(!agent2.at_front(&agent, 0));
    assert!(agent2.at_front(&agent, 1));
    assert!(agent2.at_front(&agent, 2));
    assert!(!agent2.at_front(&agent, 3));

    assert!(!agent3.at_front(&agent, 1));
    assert!(agent3.at_front(&agent, 2));
    assert!(agent3.at_front(&agent, 3));
    assert!(!agent3.at_front(&agent, 4));

    assert!(!agent4.at_front(&agent, 3));
    assert!(agent4.at_front(&agent, 4));
    assert!(agent4.at_front(&agent, 5));
    assert!(!agent4.at_front(&agent, 6));

    assert!(!agent5.at_front(&agent, 4));
    assert!(agent5.at_front(&agent, 5));
    assert!(agent5.at_front(&agent, 6));
    assert!(!agent5.at_front(&agent, 7));

    assert!(!agent6.at_front(&agent, 5));
    assert!(agent6.at_front(&agent, 6));
    assert!(agent6.at_front(&agent, 7));
    assert!(!agent6.at_front(&agent, 0));

    assert!(!agent7.at_front(&agent, 5));
    assert!(agent7.at_front(&agent, 6));
    assert!(agent7.at_front(&agent, 7));
    assert!(agent7.at_front(&agent, 0));
    assert!(!agent7.at_front(&agent, 1));
}

}