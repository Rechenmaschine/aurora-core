mod camera_controller;
pub mod coordinate_systems;
mod memory_profiler;
mod terrain;
mod trajectory;

use crate::camera_controller::{camera_controller, CameraController};
use crate::coordinate_systems::{enu_to_engine, CAMERA_START_IN_ENU, CAMERA_START_TARGET_POS};
use crate::terrain::TerrainLoaderPlugin;
use crate::trajectory::TrajectoryViewerPlugin;
use bevy::{core_pipeline::tonemapping::Tonemapping, prelude::*};
use std::hash::{Hash, Hasher};

use crate::memory_profiler::MemoryProfilerPlugin;

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            transform: Transform::from_translation(enu_to_engine(CAMERA_START_IN_ENU))
                .looking_at(enu_to_engine(CAMERA_START_TARGET_POS), Vec3::Y),
            ..default()
        },
        CameraController::default(),
    ));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    });

    commands.insert_resource(ClearColor(Color::DARK_GRAY));
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            TerrainLoaderPlugin,
            TrajectoryViewerPlugin,
            MemoryProfilerPlugin,
        ))
        .add_systems(Startup, setup_scene)
        .add_systems(Update, camera_controller)
        .run();
}
