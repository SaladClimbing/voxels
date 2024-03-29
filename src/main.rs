#![allow(non_snake_case)]
#![allow(unused_imports)]

use bevy::{
    pbr::wireframe::{NoWireframe, Wireframe, WireframeColor, WireframeConfig, WireframePlugin},
    prelude::*,
    render::{
        render_resource::WgpuFeatures,
        settings::{RenderCreation, WgpuSettings},
        RenderPlugin,
    },
    transform::commands,
};

mod camera;
mod demo;

fn light(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle { 
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    features: WgpuFeatures::POLYGON_MODE_LINE,
                    ..default()
                }),
                ..default()
            }),
            WireframePlugin,
        ))
        .add_systems(Startup, camera::setup_camera)
        .add_systems(Startup, light)
        .add_systems(Startup, demo::demo_cube_plane)
        .run();
}
