use crate::coordinate_systems::ned_to_engine;
use bevy::prelude::shape::Cylinder;
use bevy::prelude::*;
use bevy::utils::synccell::SyncCell;
use std::sync::mpsc::Receiver;
use std::thread;
use nicollier_gnc::SystemState;
use nicollier_gnc::guidance::double_wall::DoubleWallGuidance;
use nicollier_gnc::model::three_dof::ThreeDof;
use nicollier_gnc::controller::p_controller::PController;
use nicollier_gnc::model::Model;
use nicollier_gnc::controller::Controller;
use nicollier_gnc::guidance::Guidance;
use nicollier_gnc::guidance::constant_guidance::ConstantGuidance;
use nicollier_gnc::Reference;

static TRAJ_SEGMENT_RADIUS: f32 = 1.0;

pub struct TrajectoryViewerPlugin;

/// Computes a transform that transforms a cylinder with radius 1 and height 1,
/// located at the origin, such that it starts at start_coords, ends at end_coords
/// and has radius TRAJ_SEGMENT_RADIUS. start_coords and end_coords are expected in the NED coordinate frame
fn create_trajectory_segment(start_coords: Vec3, end_coords: Vec3) -> Transform {
    let start_coords = ned_to_engine(start_coords);
    let end_coords = ned_to_engine(end_coords);

    let seg = end_coords - start_coords;
    let seg_len = seg.length();

    let scale = Vec3::new(TRAJ_SEGMENT_RADIUS, seg_len, TRAJ_SEGMENT_RADIUS);
    let rotation = Quat::from_rotation_arc(Vec3::Y, seg.normalize());
    let translation = start_coords;

    Transform {
        translation,
        rotation,
        scale,
    }
}

#[derive(Resource)]
struct TrajectoryViewerResources {
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    new_segment_queue: SyncCell<Receiver<(Vec3, Vec3)>>,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = materials.add(StandardMaterial {
        base_color: Color::RED,
        alpha_mode: AlphaMode::Opaque,
        unlit: true,
        ..default()
    });

    let mesh = meshes.add(
        Cylinder {
            radius: 1.0,
            height: 1.0,
            resolution: 8,
            segments: 4,
        }
        .into(),
    );

    let (send, recv) = std::sync::mpsc::channel();

    thread::spawn(move || {

        let delta_t = 0.01;

        let initial_state = SystemState::initial_state();

        let mut guidance = ConstantGuidance::new(Reference(0.0));
        //let mut guidance = ConstantYawGuidance::new(constant_yaw_angle);//my const guidance
        // let mut guidance = DoubleWallGuidance::new(
        //     60.0,
        //     10.0,
        //     5.0,
        //     100.0,
        //     0.5
        // );
        let mut controller = PController::new();
        let mut model = ThreeDof::new(initial_state);

        while !model.landed() {
            let state = model.get_state();
            let reference = guidance.get_reference(state);
            let control_inputs = controller.step(model.get_state(), reference, delta_t);
            let updated_state = model.step(control_inputs, delta_t);

            let old_pos = state.inertial_frame_position;
            let new_pos = updated_state.inertial_frame_position;

            send.send((Vec3::new(old_pos.x as f32, old_pos.y as f32, old_pos.z as f32),
                       Vec3::new(new_pos.x as f32, new_pos.y as f32, new_pos.z as f32)))
                .expect("Failed to send data for new trajectory segment");
        }
    });

    commands.insert_resource(TrajectoryViewerResources {
        material,
        mesh,
        new_segment_queue: SyncCell::new(recv),
    })
}

fn update_trajectory(mut commands: Commands, mut traj_assets: ResMut<TrajectoryViewerResources>) {
    let mesh = traj_assets.mesh.clone();
    let material = traj_assets.material.clone();

    for (start, end) in traj_assets.new_segment_queue.get().try_iter() {
        commands.spawn(PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform: create_trajectory_segment(start, end),
            ..default()
        });
    }
}

impl Plugin for TrajectoryViewerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_trajectory);
    }
}
