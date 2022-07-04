#[allow(dead_code)]
use bevy::prelude::*;
use boid::BoidPlugin;

mod boid;
mod components;

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
}
