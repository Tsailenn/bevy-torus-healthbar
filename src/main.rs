#![allow(clippy::redundant_field_names)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

#[allow(unused_imports)]
use lib::RadialBar2D;
pub mod lib;

fn main() {}

// use bevy::{
//     prelude::*,
//     render::camera::ScalingMode,
//     sprite::{MaterialMesh2dBundle, Mesh2dHandle},
//     window::PresentMode,
// };
// use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};
// use torus_healthbar::TorusHealthbar2D;

// pub const CLEAR: Color = Color::rgb(0.3, 0.3, 0.3);
// pub const HEIGHT: f32 = 900.0;
// pub const RESOLUTION: f32 = 16.0 / 9.0;

// fn main() {
//     App::new()
//         .insert_resource(ClearColor(CLEAR))
//         .insert_resource(WindowDescriptor {
//             width: HEIGHT * RESOLUTION,
//             height: HEIGHT,
//             title: "Bevy Template".to_string(),
//             present_mode: PresentMode::Fifo,
//             resizable: false,
//             ..Default::default()
//         })
//         .add_plugins(DefaultPlugins)
//         .insert_resource(WorldInspectorParams {
//             enabled: false,
//             ..Default::default()
//         })
//         .add_plugin(WorldInspectorPlugin::new())
//         .add_startup_system(spawn_camera)
//         .add_system(toggle_inspector)
//         .add_startup_system(test_spawn_healthbar)
//         //.add_system(test_damage)
//         // .add_startup_system(torus)
//         // .add_system(rotate_torus)
//         .run();
// }

// fn spawn_camera(mut commands: Commands) {
//     let mut camera = OrthographicCameraBundle::new_2d();

//     camera.orthographic_projection.right = 1.0 * RESOLUTION;
//     camera.orthographic_projection.left = -1.0 * RESOLUTION;

//     camera.orthographic_projection.top = 1.0;
//     camera.orthographic_projection.bottom = -1.0;

//     camera.orthographic_projection.scaling_mode = ScalingMode::None;

//     commands.spawn_bundle(camera);
// }

// fn toggle_inspector(
//     input: ResMut<Input<KeyCode>>,
//     mut window_params: ResMut<WorldInspectorParams>,
// ) {
//     if input.just_pressed(KeyCode::Grave) {
//         window_params.enabled = !window_params.enabled
//     }
// }

// #[allow(dead_code)]
// fn slow_down() {
//     std::thread::sleep(std::time::Duration::from_secs_f32(1.000));
// }

// #[allow(dead_code)]
// fn test_spawn_healthbar(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     let torus_health = TorusHealthbar2D::new(
//         0.1,
//         0.2,
//         360.,
//         360.,
//         18,
//         Color::MIDNIGHT_BLUE,
//         &mut meshes,
//         &mut materials,
//     );

//     commands
//         .spawn_bundle(MaterialMesh2dBundle {
//             mesh: torus_health.mesh_handle.clone(),
//             material: torus_health.color_handle.clone(),
//             ..default()
//         })
//         .insert(torus_health);
// }

// #[allow(dead_code, unused_parens)]
// fn test_damage(
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut health_torus_query: Query<(&mut TorusHealthbar2D)>,
//     //time: Res<Time>,
// ) {
//     for mut i in health_torus_query.iter_mut() {
//         i.add_health(&mut meshes, -0.5);
//     }
// }
