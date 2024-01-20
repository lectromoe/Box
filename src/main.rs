use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use bevy_rapier3d::prelude::*;
use boxy::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    level: Level::INFO,
                    filter: "wgpu=error,bevy_render=info,bevy_ecs=error".to_string(),
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        fit_canvas_to_parent: true,
                        title: "boxy".to_string(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(BoxyCameraPlugin)
        .add_plugins(BoxyControllerPlugin)
        .add_systems(Startup, build_map)
        .run()
}

fn build_map(mut commands: Commands) {
    commands.spawn(Collider::cuboid(100.0, 1.0, 100.0));
    commands.spawn(SpotLightBundle::default());
}
