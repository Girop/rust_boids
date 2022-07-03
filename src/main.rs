#![allow(unused)]
use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

// const BOID_POPULATION: i32 = 30;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Boiiiiids".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(spawn_boid)
        .run();
}

fn setup(mut commands: Commands) {
    // Create camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_boid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(triangle_mesh())),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        ..Default::default()
    });
}

fn triangle_mesh() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleStrip);

    let position = vec![[0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0]];
    let indices = vec![0, 1, 2];

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, position);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh
}

fn movement_system() {}
