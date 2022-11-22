use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use bevy_rapier3d::prelude::*;
use boxxed::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: 1280.,
                        height: 720.,
                        scale_factor_override: Some(1.),
                        title: "boxxed".to_string(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .set(LogPlugin {
                    level: Level::INFO,
                    filter: "wgpu=error,bevy_render=info,bevy_ecs=error".to_string(),
                }),
        )
        .add_plugin(DebugCameraPlugin)
        .add_plugin(CharacterControllerPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(build_map)
        .run()
}

fn build_map(mut commands: Commands) {
    commands.spawn(Collider::cuboid(100.0, 1.0, 100.0));
    commands.spawn(SpotLightBundle::default());
}
