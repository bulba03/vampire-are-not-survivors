use bevy::{
    app::{ Plugin, Update }, ecs::{
        bundle::Bundle, component::Component, entity::Entity, query::With, schedule::IntoSystemConfigs, system::{ Commands, Query, Res }
    }, log::info, math::{vec3, Vec3}, reflect::Reflect, render::color::Color, sprite::{Sprite, SpriteBundle}, time::{Time, Timer}, transform::components::Transform
};
use bevy_xpbd_2d::plugins::collision::{Collider, CollidingEntities, Sensor};

use crate::{ general::health::Health, monster::Monster, player::attack::AttackType };

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (handle_projectile_translation, handle_projectile_collision, handle_projectile_lifetime).chain()
        );
    }
}
#[derive(Component)]
pub struct ProjectileLifeTimer(pub Timer);

#[derive(Bundle)]
pub struct AttackBundle {
    sprite: SpriteBundle,
    projectile: Projectile,
    sensor: Sensor,
    projectile_collider: ProjectileCollider,
    collider: Collider,
    lifetime: ProjectileLifeTimer
}

impl AttackBundle {
    pub fn primary(direction: Vec3, pl_pos: &Transform) -> Self {
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {color: Color::RED, ..Default::default()},
                transform: Transform {
                    translation: vec3(pl_pos.translation.x, pl_pos.translation.y, 1.),
                    scale: Vec3::splat(3.),
                    ..Default::default()
                },
                ..Default::default()
            },
            projectile: Projectile{
                effect: ProjectileEffect::None,
                damage: 10.,
                direction,
                speed: 25.0
            },
            sensor: Sensor,
            projectile_collider: ProjectileCollider,
            collider: Collider::rectangle(3., 5.),
            lifetime: ProjectileLifeTimer(Timer::from_seconds(2., bevy::time::TimerMode::Once))
        }
    }

    pub fn secondary(direction: Vec3, pl_pos: &Transform) -> Self {
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {color: Color::GREEN, ..Default::default()},
                transform: Transform {
                    translation: vec3(pl_pos.translation.x, pl_pos.translation.y, 1.),
                    scale: Vec3::splat(3.),
                    ..Default::default()
                },
                ..Default::default()
            },
            projectile: Projectile{
                effect: ProjectileEffect::None,
                damage: 10.,
                direction,
                speed: 5.0
            },
            sensor: Sensor,
            projectile_collider: ProjectileCollider,
            collider: Collider::rectangle(1., 8.),
            lifetime: ProjectileLifeTimer(Timer::from_seconds(0.5, bevy::time::TimerMode::Once))
        }
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

pub fn construct_projectile_from_attack_type(attack_type: AttackType, direction: Vec3, pl_pos: &Transform) -> AttackBundle {
    info!("attack spawned");
    match attack_type {
        AttackType::Primary => AttackBundle::primary(direction, pl_pos),
        AttackType::Secondary => AttackBundle::secondary(direction, pl_pos),
    }
}

pub fn handle_projectile_lifetime(mut commands: Commands, mut query: Query<(Entity, &mut ProjectileLifeTimer), With<ProjectileCollider>>, time: Res<Time>) {
    for (entity, mut timer) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            commands.entity(entity).despawn()
        }
    }
}

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
                    return;
                }
            }
        }
    }
}
