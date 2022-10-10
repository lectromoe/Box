mod actions;
mod camera;

use actions::*;
use bevy::prelude::*;
use camera::*;
use leafwing_input_manager::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1280.,
            height: 720.,
            scale_factor_override: Some(1.),
            title: "boxxed".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_plugin(DebugCameraPlugin)
        .add_startup_system(setup)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 8.0 })),
        material: materials.add(Color::rgb(1., 0.9, 0.9).into()),
        transform: Transform::from_translation(Vec3::new(4., 0., 4.)),
        ..Default::default()
    });
    commands.spawn_bundle(SpotLightBundle::default());
}