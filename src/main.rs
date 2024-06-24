use bevy::{ prelude::*, tasks::futures_lite::stream::Once, window::{ PresentMode, WindowTheme } };
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_2d::{ math::Vector, resources::Gravity };
use general::{damage_timer::DamageTimer, health::HealthBar};
use player::Player;
use sickle_ui::*;
use ui_builder::{UiBuilderExt, UiRoot};
use widgets::{column::UiColumnExt, container::UiContainerExt, row::UiRowExt};
pub mod player;
pub mod animation;
pub mod camera;
pub mod monster;
pub mod general;
pub mod projectiles;
fn main() {
    default_run();
}

pub fn default_run() {
    App::new()
    .add_plugins(
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "I am a window!".into(),
                name: Some("bevy.app".into()),
                resolution: (1920.0, 1080.0).into(),
                present_mode: PresentMode::AutoVsync,
                prevent_default_event_handling: false,
                window_theme: Some(WindowTheme::Dark),
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                mode: bevy::window::WindowMode::Fullscreen,
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
    .add_plugins(SickleUiPlugin)
    .add_systems(Startup, setup)
    .run();
}

fn run_if_player_alive(pl_q: Query<&Player>) -> bool {
    pl_q.get_single().is_ok()
}

fn setup(mut commands: Commands) {
    commands.ui_builder(UiRoot).container(
        (
            NodeBundle {
                style: Style {
                    width: Val::Px(250.0),
                    height: Val::Px(100.0),
                    flex_direction: FlexDirection::Column,
                    flex_wrap: FlexWrap::Wrap,
                    flex_grow: 1.,
                    justify_self: JustifySelf::Center,
                    align_self: AlignSelf::End,
                    ..default()
                },
                background_color: Color::DARK_GRAY.into(),
                ..default()
            },
        ), |container| {
            let mut s = container.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(70.0),
                    height: Val::Px(100.0),
                    flex_direction: FlexDirection::Column,
                    flex_wrap: FlexWrap::Wrap,
                    flex_grow: 1.,
                    justify_self: JustifySelf::Start,
                    align_self: AlignSelf::Start,
                    ..default()
                },
                background_color: Color::RED.into(),
                ..default()
            });
            // s.insert(Heal);
        });
  }