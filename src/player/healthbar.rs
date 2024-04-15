use bevy::transform::components::Transform;
use bevy::sprite::{Sprite, SpriteBundle};
use bevy::render::color::Color;
use bevy::prelude::default;
use bevy::math::{Vec2, Vec3};
use bevy::hierarchy::BuildChildren;
use bevy::ecs::{component::Component, entity::Entity, query::{Changed, With}, system::{Commands, Query}};

use super::Health;
const HEALTHBAR_LAYER: f32 = 90.1;
const HEALTHBAR_LAYER_BG: f32 = 90.;
#[derive(Component)]
pub struct HealthBar {
    pub size: Vec2,
    pub offset: Vec2,
}

#[derive(Component)]
pub struct HealthBarBar;

impl Default for HealthBar {
    fn default() -> Self {
        Self {
            size: Vec2::new(50.,10.),
            offset: Vec2::new(0., -35.)
        }
    }
}

pub fn spawn_healthbar(mut commands: Commands, player: Entity) {
    let bar = HealthBar::default();
    let bg_bar =  commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::BLACK,
            ..default()
        },
        transform: Transform {
            translation: bar.offset.extend(HEALTHBAR_LAYER_BG),
            scale: Vec3::new(bar.size.x + 2., bar.size.y + 2., 1.),
            ..default()
        },
        ..default()
    }).id();
    let hp_bar = commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            ..default()
        },
        transform: Transform {
            translation: bar.offset.extend(HEALTHBAR_LAYER),
            scale: bar.size.extend(1.),
            ..default()
        },
        ..default()
    },HealthBarBar)).id();
    commands.entity(player).push_children(&[hp_bar,bg_bar]);
}

pub fn update_health_bar(
        mut _commands: Commands, 
        mut pl_health_query: Query<(&Health, &HealthBar), Changed<Health>>,
        mut hp_bar_query: Query<&mut Transform , With<HealthBarBar>>)
    {
        for (health, hp_bar) in pl_health_query.iter_mut(){
            let mut transform = hp_bar_query.single_mut();
            let frac = (health.current /health.max).clamp(0., 1.);
            let current_width = frac * hp_bar.size.x;

            transform.translation.x = (hp_bar.size.x - current_width) / -2.;
            transform.scale.x = current_width
        }
}