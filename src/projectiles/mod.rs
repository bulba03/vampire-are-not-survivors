use bevy::{
    app::{ Plugin, Update },
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        schedule::IntoSystemConfigs,
        system::{ Commands, Query, Res },
    },
    log::info,
    math::Vec3,
    reflect::Reflect,
    time::Time,
    transform::components::Transform,
};
use bevy_xpbd_2d::plugins::collision::CollidingEntities;

use crate::{ monster::Monster, general::health::Health };

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (handle_projectile_translation, handle_projectile_collision).chain()
        );
    }
}
#[derive(Component)]
pub struct Projectile {
    pub effect: ProjectileEffect,
    pub damage: f32,
    pub direction: Vec3,
    pub speed: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProjectileEffect {
    #[default]
    None,
    Fire,
}
#[derive(Component, Default, Reflect)]
pub struct ProjectileCollider;

pub fn handle_projectile_translation(
    mut query: Query<(&mut Transform, &Projectile)>,
    time: Res<Time>
) {
    for (mut transform, projectile) in query.iter_mut() {
        transform.translation += projectile.direction * projectile.speed * time.delta_seconds();
    }
}

pub fn handle_projectile_collision(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Projectile, &CollidingEntities), With<ProjectileCollider>>,
    mut m_q: Query<(Entity, &mut Health), With<Monster>>
) {
    for (proj_ent, projectile, colliding_entities) in &mut query {
        if colliding_entities.0.is_empty() {
            continue;
        } else {
            for (entity, mut health) in m_q.iter_mut() {
                if colliding_entities.0.get(&entity).is_some() {
                    health.deal_damage(projectile.damage);
                    let is_alive = health.is_alive();
                    if is_alive == false {
                        commands.entity(entity).despawn();
                    }
                    commands.entity(proj_ent).despawn();
                }
            }
        }
    }
}
