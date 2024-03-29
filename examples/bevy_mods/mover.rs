use rand::Rng;
use bevy::prelude::*;
use super::*;



#[derive(Component, Clone)]
pub struct Mover {
    pub direction: usize,
    pub speed: f32,
    pub duration: f32,
    pub timer: Timer,
    pub pause: bool,
}

impl Default for Mover {

    fn default() -> Self {
        
        Self {
            direction: 0, 
            speed: 0.0, 
            duration: 0.0, 
            timer: Timer::default(),
            pause: false,
        }
    }
}


impl Mover {

    pub fn new() -> Self {

        let seconds = rand_duration();

        Self {
            direction: rand_direction(),

            speed: rand_speed(),

            duration: seconds,

            timer: Timer::from_seconds(seconds, TimerMode::Once),

            pause: false,
        }
    }

    pub fn random(&mut self) {
        self.direction = rand_direction();
        self.speed = rand_speed();
        self.duration = rand_duration();
        self.timer = Timer::from_seconds(self.duration, TimerMode::Once);
    }

    pub fn back(&mut self, back: u8) {
    
        let range: i32 = rand::thread_rng().gen_range(-1..2);
        self.direction = (back as i32 + range + DIRECTIONS as i32) as usize % DIRECTIONS;
    }

    pub fn bump(&mut self) {
    
        let range: i32 = rand::thread_rng().gen_range(-2..3);
        self.direction = (self.direction as i32 + range + DIRECTIONS as i32) as usize % DIRECTIONS;
    }

    pub fn dodge(&mut self, dirs:&Vec<usize>) {

        if dirs.contains(&self.direction) {
            return;
        }

        let index = rand::thread_rng().gen_range(0..dirs.len());
        self.direction = dirs[index];
    }

    pub fn stop(&mut self) {

        self.pause = true;
    }

}

pub fn rand_direction() -> usize {

    rand::thread_rng().gen_range(0..DIRECTIONS)
}

pub fn rand_speed() -> f32 {

    rand::thread_rng().gen_range(MIN_SPEED..MAX_SPEED)
}

pub fn rand_duration() -> f32 {

    rand::thread_rng().gen_range(MIN_DURATION..MAX_DURATION)
}


