#[allow(dead_code)]
use bevy::prelude::*;
use boid::BoidPlugin;

mod boid;
mod components;

const BASE_SPEED: i32 = 1; // and unused
const TIME_STEP: f32 = 0.1 / 60.; //Unused right now
const MAX_SPEED: i32 = 10; // also unused

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Boids".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(BoidPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    // BG color
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.insert_resource(BASE_SPEED);
    commands.insert_resource(TIME_STEP);
    commands.insert_resource(MAX_SPEED);
}
