#![allow(unused)]
use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
    sprite::MaterialMesh2dBundle,
};

// const BOID_POPULATION: i32 = 30;

// Entities

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
        mesh: meshes.add(triangle_mesh()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        ..Default::default()
    });
}

fn triangle_mesh() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let position = vec![[0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0]];
    let normals = vec![[0.0, 0.0, 0.0], [0.0, 0.0, 0.0],[0.0,0.0,1.0]]; // is this just normal vector ?
    let uvs = vec![[0.0,0.0,0.1],[0.0,0.0,0.0],[0.0,0.0,0.0]]; // wtf is this / some shit about texture coordinates  <== again what kind of shit is this ? 

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, position);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    mesh.set_indices(Some(Indices::U32(vec![0, 1, 2])));

    mesh
}
