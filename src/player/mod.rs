mod movement;
mod healthbar;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::ecs::entity::Entity;
use bevy::ecs::event::EventReader;
use bevy::ecs::query::With;
use bevy::ecs::system::Query;
use bevy::log::info;
use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::reflect::Reflect;
use bevy::render::color::Color;
use bevy::render::primitives::Aabb;
use bevy::time::{Timer, TimerMode};
use bevy::transform::components::Transform;
use bevy::sprite::{Sprite, SpriteSheetBundle, TextureAtlas, TextureAtlasLayout};
use bevy::prelude::default;
use bevy::math::{Vec2, Vec3};
use bevy::ecs::{component::Component, system::{Commands, Res, ResMut}};
use bevy::asset::{AssetServer, Assets};
use bevy::app::{Plugin, PostUpdate, Startup, Update};
use bevy_xpbd_2d::components::{ColliderDensity, LockedAxes, RigidBody};
use bevy_xpbd_2d::plugins::collision::contact_reporting::{CollisionEnded, CollisionStarted};
use bevy_xpbd_2d::plugins::collision::{Collider, CollidingEntities, Sensor};
use bevy_xpbd_2d::plugins::PhysicsPlugins;
use crate::animation::{AnimationIndices, AnimationTimer};
use self::healthbar::{spawn_healthbar, update_health_bar, HealthBar};
use self::movement::handle_movement;

const SKELETON_WALK_ANIM: &str = "monster/Monsters_Creatures_Fantasy/Skeleton/Walk.png";

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_player)
        .add_plugins(           ( PhysicsPlugins::default(),
        FrameTimeDiagnosticsPlugin))
        .add_systems(Update, handle_movement)
        .add_systems(PostUpdate, (update_health_bar,apply_pressure_plate_colour));
    }
}


#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub base_damage: f32,
    pub is_moving: bool,
}
#[derive(Component)]
pub struct Health {
    pub max: f32,
    pub current: f32
}
fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
    ) { 
        let texture = asset_server.load(SKELETON_WALK_ANIM);
        let layout = TextureAtlasLayout::from_grid(Vec2::new(150.0, 150.0), 4, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_indices = AnimationIndices { first: 0, last: 3 };
        
        let entity = commands.spawn(
            (   
                SpriteSheetBundle{
                    texture,
                    atlas: TextureAtlas {
                        layout: texture_atlas_layout,
                        index: animation_indices.first,
                    },
                    transform: Transform::from_scale(Vec3::splat(1.)),
                    ..default()
                
                },
                Player {
                    speed:10000.0,
                    base_damage: 10.,
                    is_moving: false
                },
                Health {
                    max: 100.,
                    current: 100.
                },
                animation_indices,
                AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                HealthBar::default(),
                Collider::rectangle(20.,20.),
                LockedAxes::ALL_LOCKED,
                RigidBody::Dynamic,
                
                // ColliderDensity::ZERO,
                TestCol
            )
        ).id();
        spawn_healthbar(commands, entity);
}

#[derive(Component, Default, Reflect)]
pub struct TestCol;

fn apply_pressure_plate_colour(
    mut commands: Commands,
    mut query: Query<(Entity,&mut Sprite, &CollidingEntities), With<TestCol>>,
) {
    for (mut en, mut sprite, colliding_entities) in &mut query {
        if colliding_entities.0.is_empty() {
            sprite.color = Color::BLUE;
        } else {
            sprite.color = Color::RED;
            info!("COLLISION");
        }

    }
}
fn log_events(mut started: EventReader<CollisionStarted>, mut ended: EventReader<CollisionEnded>) {
    // print out the started and ended events
    for event in started.read() {
        println!("CollisionStarted: {:?}", event);
    }
    for event in ended.read() {
        println!("CollisionEnded: {:?}", event);
    }
}