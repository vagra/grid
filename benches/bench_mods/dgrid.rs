use rand::{Rng, rngs::StdRng};
use grid::{
    *,
    dgrid::{*, agent::*}
};
use super::*;


#[derive(Default)]
pub struct Actor {
    pub index: u16,
    pub id: u32,
    pub prev_x: f32,
    pub prev_y: f32,
    pub x: f32,
    pub y: f32,
    pub hw: i16,
    pub hh: i16,

    pub speed: f32,
    pub dir: usize,
    pub duration: u16,
    pub times: u16,
    pub pause: bool,
}

impl Actor {

    pub fn new(index:u16, id:u32, x:i16, y:i16, hw:i16, hh:i16) -> Self {

        Self {
            index,
            id,

            prev_x: x as f32,
            prev_y: y as f32,
            x: x as f32,
            y: y as f32,

            hw,
            hh,

            ..Default::default()
        }
    }

    pub fn rand_pos(grid: &DGrid, rng: &mut StdRng) -> (i16, i16) {
        
        (   rng.gen_range( -grid.half_width..grid.half_width ),
            rng.gen_range( -grid.half_height..grid.half_height ) )
    }

    pub fn rand_size(rng: &mut StdRng) -> (i16, i16) {
        
        (   rng.gen_range( 0..MIN_RADIUS ),
            rng.gen_range( 0..MAX_RADIUS ) )
    }

    pub fn rand_speed(rng: &mut StdRng) -> f32{

        rng.gen_range(MIN_SPEED..MAX_SPEED)
    }

    pub fn rand_dir(rng: &mut StdRng) -> usize{

        rng.gen_range(0..DIRECTIONS) as usize
    }

    pub fn rand_duration(rng: &mut StdRng) -> u16 {

        rng.gen_range(MIN_DURATION..MAX_DURATION)
    }

    pub fn rand(&mut self, rng: &mut StdRng) {

        self.speed = Self::rand_speed(rng);
        self.dir = Self::rand_dir(rng);
        self.duration = Self::rand_duration(rng);
        self.times = 0;
        self.pause = false;
    }
    
    pub fn move_step(&mut self, rng: &mut StdRng) {

        self.prev_x = self.x;
        self.prev_y = self.y;

        if self.pause {

            self.pause = false;
            return;
        }

        self.times += 1;

        if self.times >= self.duration {

            self.rand(rng);
            return;
        }

        let offset = VECTORES[self.dir];

        self.x += self.speed * offset.x;
        self.y += self.speed * offset.y;
    }
    

    pub fn query_bump(&mut self, grid: &DGrid, rng: &mut StdRng) {

        let dirs = grid.query_dirs(self.x as i16, self.y as i16, self.hw, self.hh, self.id,);

        if dirs.len() == 8 {

            return;
        }

        if dirs.contains(&self.dir) {

            return;
        }

        if dirs.len() == 0 {

            self.pause;
            return;
        }

        self.dodge(&dirs, rng);
    }

    pub fn query_out(&mut self, grid: &DGrid, rng: &mut StdRng) {

        if let Some(dir) = grid.out_bounds(self.x as i16, self.y as i16) {

            self.back(dir, rng);
        }
    }

    pub fn back(&mut self, back: u8, rng: &mut StdRng) {
    
        let range: i32 = rng.gen_range(-1..2);
        self.dir = (back as i32 + range + DIRECTIONS as i32) as usize % DIRECTIONS as usize;
    }

    pub fn dodge(&mut self, dirs:&Vec<usize>, rng: &mut StdRng) {

        if dirs.contains(&self.dir) {
            return;
        }

        let index = rng.gen_range(0..dirs.len());
        self.dir = dirs[index];
    }
}


pub fn bench_dgrid(count:usize) {

    let rng = &mut init_seed();
    let grid = &mut create_grid(count, rng);
    let actors = &mut create_actors(&grid, rng);

    for _ in 0..FRAMES {
        
        move_actors(actors, grid, rng);
        turn_actors(actors, grid, rng);
    }
}


pub fn create_grid(count: usize, rng: &mut StdRng) -> DGrid {

    let mut grid = DGrid::new(4, 20, 20, 20);

    let mut x: i16;
    let mut y: i16;
    let mut hw: i16;
    let mut hh: i16;
    for i in 0..count {

        (x, y) = Actor::rand_pos(&grid, rng);
        (hw, hh) = Actor::rand_size(rng);

        grid.insert(i as u32, x, y, hw, hh);
    }

    grid
}

pub fn create_actors(grid: &DGrid, rng: &mut StdRng) -> Vec<Actor>{

    let mut actors: Vec<Actor> = Vec::new();

    let mut agent: &Agent;
    for index in 0..grid.loose.pool.capacity() {

        agent = &grid.loose.pool[index];

        if agent.is_free() {

            continue;
        }

        let mut actor = Actor::new(index, agent.id, agent.x, agent.y, agent.hw, agent.hh);
        actor.rand(rng);

        actors.push(actor);
    } 

    actors
}


pub fn move_actors(actors: &mut Vec<Actor>, grid: &mut DGrid, rng: &mut StdRng) {

    let mut actor: &mut Actor;
    let mut agent: &Agent;
    for index in 0..actors.len() {

        actor = &mut actors[index];

        actor.move_step(rng);

        agent = &grid.loose.pool[actor.index];

        assert!(agent.id != INACTIVE);
        assert!(agent.id == actor.id);

        grid.move_cell(
            actor.id, actor.prev_x as i16, actor.prev_y as i16,
            actor.x as i16, actor.y as i16);
    }
}



pub fn turn_actors(actors: &mut Vec<Actor>, grid: &mut DGrid, rng: &mut StdRng) {

    let mut actor: &mut Actor;
    for index in 0..actors.len() {

        actor = &mut actors[index];

        actor.query_bump(grid, rng);
        actor.query_out(grid, rng);
    }
}


