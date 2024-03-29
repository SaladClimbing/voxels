#![allow(non_snake_case)]

use bevy::{a11y::accesskit::Point, pbr, prelude::*};

// NOTE
// I don't know what this does entirely
// Rust syntax is still new
// https://bevy-cheatbook.github.io/3d/camera.html
// TODO Move to seperate file
#[derive(Component)]
struct MyCameraMarker;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(10.0, 12.0, 16.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MyCameraMarker,
    ));
}

fn demo_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // Circle base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });

    // Cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::BLUE),
        ..default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(5.0, 5.0, 5.0),
        ..default()
    });

}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, demo_cube)
        .run();
}
