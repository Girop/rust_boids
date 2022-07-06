#![allow(unused)]
use std::ops::{Add, Mul};

use bevy::prelude::*;
use rand::Rng;

use crate::components::Velocity;

const BOID_SPRITE: &str = "boid.png";
const VIEW_DISTANCE: f32 = 50.0;

pub struct BoidPlugin;

impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::Startup, spawn_boid)
            .add_startup_system_to_stage(StartupStage::PostStartup, boid_initialize_velocity)
            .add_system(boid_movement);
    }
}

#[derive(Component)]
pub struct Boid;

fn spawn_boid(mut commands: Commands, assets_server: Res<AssetServer>, windows: Res<Windows>) {
    let mut rng = rand::thread_rng();
    let window = windows.get_primary().unwrap();
    let (width, height) = (window.width(), window.height());

    for _ in 0..20 {
        let pos_x = rng.gen_range(-width..width);
        let pos_y = rng.gen_range(-height..height);

        commands
            .spawn_bundle(SpriteBundle {
                texture: assets_server.load(BOID_SPRITE),
                transform: Transform {
                    translation: Vec3::new(pos_x, pos_y, 0.0),
                    scale: Vec3::new(0.03, 0.03, 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Boid)
            .insert(Velocity { x: 0.0, y: 0.0 });
    }
}

fn separation(nearby: Vec<(f32, f32)>, this_pos: (f32, f32)) -> Vec2 {
    let mut separation_vec = Vec2::from((0.0, 0.0));

    for (x_nearby, y_nearby) in nearby {
        let reverse_vec = Vec2::from((this_pos.0 - x_nearby, this_pos.1 - y_nearby));
        separation_vec += reverse_vec;
    }
    separation_vec.normalize();

    separation_vec
}

fn aligment(nearby: Vec<(f32, f32)>) -> Vec2 {
    let mut aligment_vec = Vec2::from((0.0, 0.0));

    for (nearby_x, nearby_y) in nearby {
        aligment_vec[0] += nearby_x;
        aligment_vec[1] += nearby_y;
    }

    aligment_vec.normalize();

    aligment_vec
}

fn cohesion(nearby: Vec<(f32, f32)>,this_pos: (f32, f32)) -> Vec2 {
    let mut cohesion_vec = Vec2::from((0.0, 0.0)); 
    for nearby_position in nearby{
        cohesion_vec.add(Vec2::from(nearby_position));
    }
    
    cohesion_vec.mul(1.0/(nearby.len() as f32));
    cohesion_vec
}

fn boid_initialize_velocity(mut boids_query: Query<&mut Velocity, With<Boid>>) {
    let mut rng = rand::thread_rng();
    for mut velocity in boids_query.iter_mut() {
        let mut start_vec = Vec2::from((rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)));
        start_vec = start_vec.normalize();
        velocity.x = start_vec[0];
        velocity.y = start_vec[1];
    }
}

fn calculate_acceleration(mut boids_query: Query<(&mut Velocity, &Transform), With<Boid>>) {
    let mut population: Vec<(f32, f32)> = Vec::new();
    for (_, transform) in boids_query.iter_mut() {
        population.push((transform.translation.x, transform.translation.y));
    }
}

fn boid_movement(mut boids_query: Query<(&Velocity, &mut Transform), With<Boid>>) {
    for (velocity, mut transform) in boids_query.iter_mut() {
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
        // rotation in direcrtion of velocity / acceleration
    }
}
