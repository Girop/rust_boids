use bevy::{prelude::*, transform};
use rand::Rng;

use crate::components::Velocity;

const BOID_SPRITE: &str = "boid.png";
const VIEW_DISTANCE: f32 = 50.0;

pub struct BoidPlugin;

impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_boid);
        // .add_system(boid_movement_system);
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

struct Acceleration {
    x: f32,
    y: f32,
}

fn separation() {}

fn aligment() {}

fn cohesion() {}

fn boid_movement_system(mut boids_query: Query<(&mut Velocity, &mut Transform), With<Boid>>) {

}
