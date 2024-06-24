mod movement;
pub mod attack;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::ecs::schedule::{ IntoSystemConfigs, IntoSystemSetConfigs, SystemSet };
use bevy::time::{ Timer, TimerMode };
use bevy::transform::components::Transform;
use bevy::sprite::{ Sprite, SpriteSheetBundle, TextureAtlas, TextureAtlasLayout };
use bevy::prelude::{ default, App };
use bevy::math::{ vec2, Vec2, Vec3 };
use bevy::ecs::{ component::Component, system::{ Commands, Res, ResMut } };
use bevy::asset::{ AssetServer, Assets };
use bevy::app::{ Plugin, PostStartup, Startup, Update };
use bevy_xpbd_2d::components::{ LockedAxes, RigidBody };
use bevy_xpbd_2d::plugins::collision::Collider;
use bevy_xpbd_2d::plugins::{ PhysicsDebugPlugin, PhysicsPlugins };
use crate::animation::{ AnimationIndices, AnimationTimer };
use crate::general::damage_timer::DamageTimer;
use crate::general::health::{spawn_healthbar, Health, HealthBar};
use crate::general::sheets_holder::SheetsHolder;
use crate::general::GeneralSet;
use crate::run_if_player_alive;
use self::attack::{ player_attack, handle_attack_pressed, AttackEvent };
use self::movement::handle_movement;

const SKELETON_WALK_ANIM: &str = "monster/Monsters_Creatures_Fantasy/Skeleton/Walk.png";

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player.after(GeneralSet))
            .add_plugins((
                PhysicsPlugins::default(),
                // PhysicsDebugPlugin::default(),
                FrameTimeDiagnosticsPlugin,
            ))
            .add_event::<AttackEvent>()
            .configure_sets(Update, MyInputSet.run_if(run_if_player_alive).after(GeneralSet))
            .add_systems(
                Update,
                (handle_movement, handle_attack_pressed, player_attack)
                    .run_if(run_if_player_alive)
                    .chain()
                    .in_set(MyInputSet)
            );
    }
}
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MyInputSet;
#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub base_damage: f32,
    pub is_moving: bool,
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    pl_sprites: Res<SheetsHolder>
) {
    let texture = pl_sprites.up.clone();
    // let layout = TextureAtlasLayout::from_grid(Vec2::new(150.0, 150.0), 4, 1, None, None);
    // let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 9 };

    let entity = commands
        .spawn((
            SpriteSheetBundle {
                texture,
                atlas: TextureAtlas {
                    layout: pl_sprites.up_texture_layout.clone(),
                    index: animation_indices.first,
                },
                sprite: Sprite {
                    custom_size: Some(Vec2 { x: 84., y: 101. }),
                    anchor: bevy::sprite::Anchor::Center,
                    ..Default::default()
                },
                
                transform: Transform::from_scale(Vec3::splat(1.0)),
                ..default()
            },
            Player {
                speed: 150.0,
                base_damage: 10.0,
                is_moving: false,
            },
            Health {
                max: 100.0,
                current: 100.0,
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
            HealthBar::default(),
            Collider::rectangle(40.0, 50.0),
            LockedAxes::ROTATION_LOCKED,
            DamageTimer(Timer::from_seconds(0.25, TimerMode::Once)),
            RigidBody::Dynamic,
        ))
        .id();
    spawn_healthbar(commands, entity);
}
