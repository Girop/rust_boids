use bevy::prelude::*;
use rand::Rng;

use crate::components::Velocity;

const BOID_SPRITE: &str = "boid.png";
pub struct BoidPlugin;

impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_boid)
            .add_system(boid_movement_system);
    }
}

#[derive(Component)]
pub struct Boid;

fn spawn_boid(mut commands: Commands, assets_server: Res<AssetServer>, windows: Res<Windows>) {
    let mut rng = rand::thread_rng();
    let window = windows.get_primary().unwrap();
    let (width, height) = (window.width(), window.height());

    let pos_x = rng.gen_range(-width..width);
    let pos_y = rng.gen_range(-height..height);
    for _ in 0..10 {
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

struct Acceleration {
    x: f32,
    y: f32,
}

struct BoidDataWrapper {
    postion: (f32, f32),
    velocity: (f32, f32),
}

impl BoidDataWrapper{
    fn from(pos: (f32,f32),vel:(f32,f32)) -> Self{
        BoidDataWrapper { postion: pos, velocity: vel }
    }
}

fn separation() {}

fn aligment() {}

fn cohesion() {}

fn boid_movement_system(mut boids_query: Query<(&mut Velocity, &mut Transform), With<Boid>>) {
    let mut population: Vec<BoidDataWrapper> = Vec::new();
    for (mut velocity_iter, mut transform) in boids_query.iter_mut() {
        let translation = &transform.translation;
        let position = (translation.x,translation.y);
        let velocity = (velocity_iter.x,velocity_iter.y);

        let boid_data = BoidDataWrapper::from(position, velocity);
        population.push(boid_data);
    }

    for (index,data) in population.iter_mut().enumerate(){
        let (position,velocity) = (data.postion,data.velocity);



        position.0 += velocity.0;
        position.1 += velocity.1;
    }
}
