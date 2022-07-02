#![allow(unused)]
use bevy::prelude::*;
use rand::Rng;

const BOID_POPULATION: i32 = 30;
const BOID_SORITE: &str = "triangle.jpg";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

struct Boid {
    x: f32,
    y: f32,
}

impl Boid {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen::<f32>();
        let y = rng.gen::<f32>();
        Boid { x: x, y: y }
    }

    pub fn live(&mut self) {
        let mut rng = rand::thread_rng();
        let x = rng.gen::<f32>();
        let y = rng.gen::<f32>();
        self.x += x;
        self.y += y;
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
