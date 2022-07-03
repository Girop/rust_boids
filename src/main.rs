#![allow(dead_code)]
use bevy::{asset, prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

const BOID_SPRITE: &str = "boid.png";
const TIME_STEP: f32 = 1./60.;

struct WinSize {
    w: f32,
    h: f32,
}

struct Boid;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "Boids".to_string(),
            ..Default::default()
        })
        .add_startup_system(setup)
        .add_startup_system(spawn_boid)
        .run();
}

fn setup(mut commands: Commands, mut windows: ResMut<Windows>) {
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let window = windows.get_primary_mut().unwrap();
    commands.insert_resource(WinSize {
        h: window.height(),
        w: window.width(),
    });
}

fn spawn_boid(mut commands: Commands, windows: Res<WinSize>, assets_server: Res<AssetServer>) {
    let mut rng = rand::thread_rng();

    for _i in 0..10 {
        let pos_x = rng.gen_range(-windows.w..windows.w);
        let pos_y = rng.gen_range(-windows.h..windows.h);

        commands.spawn_bundle(SpriteBundle {
            texture: assets_server.load(BOID_SPRITE),
            transform: Transform {
                translation: Vec3::new(pos_x, pos_y, 0.0),
                scale: Vec3::new(0.03, 0.03, 1.),
                ..Default::default()
            },
            ..Default::default()
        }).insert(Boid);
    }
}
