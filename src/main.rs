use bevy::{ prelude::*, window::{ PresentMode, WindowTheme } };
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_2d::{ math::Vector, resources::Gravity };
use player::Player;
pub mod player;
pub mod animation;
pub mod camera;
pub mod monster;
pub mod general;
pub mod projectiles;
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "I am a window!".into(),
                    name: Some("bevy.app".into()),
                    resolution: (500.0, 300.0).into(),
                    present_mode: PresentMode::AutoVsync,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    // This will spawn an invisible window
                    // The window will be made visible in the make_visible() system after 3 frames.
                    // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                    visible: true,
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins(WorldInspectorPlugin::new())

        .insert_resource(Gravity(Vector::ZERO))
        
        .add_plugins((
            animation::AnimationPlugin,
            general::GeneralSystemsPlugin,
            monster::MonsterPlugin,
            player::PlayerPlugin,
            camera::CameraPlugin,
            projectiles::ProjectilePlugin,
        ))
        .run();
}

fn run_if_player_alive(pl_q: Query<&Player>) -> bool {
    pl_q.get_single().is_ok()
}

