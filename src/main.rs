#![allow(non_snake_case)]

use bevy::prelude::*;

// I don't know what this does entirely
// Rust syntax is still new
// https://bevy-cheatbook.github.io/3d/camera.html
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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .run();
}
