#![allow(dead_code)]

use bevy::{
    pbr::wireframe::{NoWireframe, Wireframe, WireframeColor, WireframeConfig, WireframePlugin},
    prelude::*,
    render::{
        render_resource::WgpuFeatures,
        settings::{RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};

use noise::{NoiseFn, Perlin, Seedable};

pub fn plate(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Circle base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(5.0, 5.0, 5.0),
        ..default()
    });
}

pub fn demo_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::BLUE),
            ..default()
        },
        Wireframe,
        WireframeColor {
            color: Color::LIME_GREEN,
        },
    ));
}

pub fn demo_cube_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let perlin = Perlin::new(1);
    for i in 0..10 {
        for j in 0..10 {
            println!("{}", perlin.get([-i as f64 + 0.5, -j as f64 + 0.5]));
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgba(0.2, 0.7, 0.1, 1.0),
                        alpha_mode: AlphaMode::Mask(0.5),
                        ..default()
                    }),
                    transform: Transform::from_xyz(
                        -i as f32,
                        perlin.get([-i as f64 + 0.5, -j as f64 + 0.5]) as f32 * 10.0,
                        -j as f32,
                    ),
                    ..default()
                },
                // Wireframe,
                // WireframeColor {
                //     color: Color::LIME_GREEN,
                // },
            ));
        }
    }
}
