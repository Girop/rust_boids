use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::Rng;

use crate::components::Velocity;

const VIEW_DISTANCE: f32 = 125.0;
const BOID_POPULATION: i32 = 50;

const SEPARATION_FORCE: f32 = 0.02;
const COHESION_FORCE: f32 = 0.32;
const ALIGMENT_FORCE: f32 = 0.3;

pub struct BoidPlugin;

impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::Startup, spawn_boid)
            .add_startup_system_to_stage(StartupStage::PostStartup, boid_initialize_velocity)
            .add_system(calculate_acceleration.before("movement"))
            .add_system(boid_movement.label("movement"));
        // Acceleration and movement ;
    }
}

#[derive(Component)]
pub struct Boid;

fn spawn_boid(mut commands: Commands, windows: Res<Windows>) {
    let mut rng = rand::thread_rng();
    let window = windows.get_primary().unwrap();
    let (width, height) = (window.width(), window.height());

    let shape = shapes::RegularPolygon {
        sides: 3,
        feature: shapes::RegularPolygonFeature::Radius(5.0),
        ..shapes::RegularPolygon::default()
    };

    for _ in 0..BOID_POPULATION {
        let x = rng.gen_range(-width / 2.0..width / 2.0);
        let y = rng.gen_range(-height / 2.0..height / 2.0);

        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::RED),
                    outline_mode: StrokeMode::color(Color::RED),
                },
                Transform {
                    translation: Vec3::new(x, y, 0.0),
                    ..Default::default()
                },
            ))
            .insert(Boid)
            .insert(Velocity { x: 0.0, y: 0.0 });
    }
}

fn separation(nearby: &Vec<(f32, f32)>, this_pos: (&f32, &f32)) -> Vec2 {
    let mut separation_vec = Vec2::from((0.0, 0.0));

    for (x_nearby, y_nearby) in nearby.iter() {
        let mut reverse_vec = Vec2::from((this_pos.0 - *x_nearby, this_pos.1 - *y_nearby));
        reverse_vec /= reverse_vec.length().powf(2.0);
        separation_vec += reverse_vec;
    }

    separation_vec.normalize_or_zero() * SEPARATION_FORCE
}

fn aligment(nearby_velocity: &Vec<(f32, f32)>, current_velocity: &(f32, f32)) -> Vec2 {
    let mut aligment_vec = Vec2::new(0.0, 0.0);

    for velocity in nearby_velocity.iter() {
        aligment_vec += Vec2::from(*velocity);
    }
    aligment_vec += Vec2::from(*current_velocity);

    aligment_vec.normalize_or_zero() * ALIGMENT_FORCE
}

fn cohesion(nearby: &Vec<(f32, f32)>, this_pos: (&f32, &f32)) -> Vec2 {
    let mut weight_center = Vec2::new(0.0, 0.0);
    for nearby_location in nearby.iter() {
        weight_center += Vec2::from(*nearby_location);
    }
    weight_center /= nearby.len() as f32;

    let cohesion_vec = Vec2::new(weight_center.x - this_pos.0, weight_center.y - this_pos.1);
    cohesion_vec.normalize_or_zero() * COHESION_FORCE
}

fn boid_initialize_velocity(mut boids_query: Query<&mut Velocity, With<Boid>>) {
    let mut rng = rand::thread_rng();
    for mut velocity in boids_query.iter_mut() {
        let mut start_vec = Vec2::from((rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)));
        start_vec = start_vec.normalize_or_zero();

        velocity.x = start_vec[0];
        velocity.y = start_vec[1];
    }
}

fn calculate_acceleration(mut boids_query: Query<(&mut Velocity, &Transform), With<Boid>>) {
    let mut other_boids: Vec<((f32, f32), (f32, f32))> = Vec::new();
    for (vel, transform) in boids_query.iter() {
        let x = transform.translation.x.clone();
        let y = transform.translation.y.clone();

        let x_vel = vel.x.clone();
        let y_vel = vel.y.clone();

        other_boids.push(((x, y), (x_vel, y_vel)));
    }

    for (mut current_velocity, current_boid) in boids_query.iter_mut() {
        let mut nearby_position: Vec<(f32, f32)> = Vec::new();
        let mut nearby_velocity: Vec<(f32, f32)> = Vec::new();

        let mut acceleration = Vec2::new(0.0, 0.0);
        let current_x = current_boid.translation.x;
        let current_y = current_boid.translation.y;

        for ((other_x, other_y), (other_vel_x, other_vel_y)) in other_boids.iter() {
            let distance = Vec2::new(current_x - other_x, current_y - other_y).length();

            if distance < VIEW_DISTANCE && distance != 0.0 {
                nearby_position.push((*other_x, *other_y));
                nearby_velocity.push((*other_vel_x, *other_vel_y));
            }
        }
        acceleration += cohesion(&nearby_position, (&current_x, &current_y));
        acceleration += separation(&nearby_position, (&current_x, &current_y));
        acceleration += aligment(&nearby_velocity, &(current_velocity.x, current_velocity.y));

        current_velocity.x += acceleration[0];
        current_velocity.y += acceleration[1];
    }
}

fn boid_movement(
    mut boids_query: Query<(&mut Velocity, &mut Transform), With<Boid>>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    let (height, width) = (window.height(), window.width());

    for (mut velocity, mut transform) in boids_query.iter_mut() {
        let x = &mut transform.translation.x;
        if *x > width / 2.0 || *x < -width / 2.0 {
            velocity.x *= -1.0;
        }
        let true_x_vel = velocity.x.clamp(-2.0, 2.0);
        *x += true_x_vel;
        let y = &mut transform.translation.y;
        if *y > height / 2.0 || *y < -height / 2.0 {
            velocity.y *= -1.0;
        }
        let true_y_vel = velocity.y.clamp(-2.0, 2.0);
        *y += true_y_vel;
    }
}
