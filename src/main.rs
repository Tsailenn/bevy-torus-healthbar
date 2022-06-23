#![allow(clippy::redundant_field_names)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use bevy::{
    prelude::*,
    render::camera::ScalingMode,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::PresentMode,
};
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};

pub const CLEAR: Color = Color::rgb(0.3, 0.3, 0.3);
pub const HEIGHT: f32 = 900.0;
pub const RESOLUTION: f32 = 16.0 / 9.0;

#[derive(Component)]
struct Torus;

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: HEIGHT * RESOLUTION,
            height: HEIGHT,
            title: "Bevy Template".to_string(),
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(WorldInspectorParams {
            enabled: false,
            ..Default::default()
        })
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(spawn_camera)
        .add_system(toggle_inspector)
        .add_startup_system(torus)
        .add_system(rotate_torus)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;

    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}

fn toggle_inspector(
    input: ResMut<Input<KeyCode>>,
    mut window_params: ResMut<WorldInspectorParams>,
) {
    if input.just_pressed(KeyCode::Grave) {
        window_params.enabled = !window_params.enabled
    }
}

#[allow(dead_code)]
fn slow_down() {
    std::thread::sleep(std::time::Duration::from_secs_f32(1.000));
}

fn torus(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // commands.spawn_bundle(MaterialMesh2dBundle {
    //     mesh: meshes.add(
    //         shape::Torus {
    //             ..Default::default()
    //         },
    //         ..default(),
    //     ),
    //     ..default()
    // });
    let t = shape::Torus {
        radius: 0.4,
        ring_radius: 0.05,
        subdivisions_segments: 36,
        subdivisions_sides: 36,
    };

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Mesh::from(t))).into(),
            material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
            transform: Transform {
                rotation: Quat::from_rotation_x(90. * (3.14 / 180.)),
                ..default()
            },
            ..default()
        })
        .insert(Torus);
}

fn rotate_torus(mut torus_query: Query<(&mut Transform), With<Torus>>, time: Res<Time>) {
    for mut torus in torus_query.iter_mut() {
        //torus.rotate(Quat::from_rotation_x(1.));
        torus.rotate(Quat::from_rotation_x(3.14 / 180.));
    }
}
