use bevy::{
    ecs::{
        bundle::Bundle, component::Component, event::{ Event, EventReader, EventWriter }, system::{ Commands, Query, Res, ResMut }
    },
    input::{ mouse::MouseButton, ButtonInput },
    log::info,
    math::{ vec3, Vec2, Vec3 },
    render::color::Color,
    sprite::{ Sprite, SpriteBundle },
    time::Timer,
    transform::components::Transform,
};
use bevy_xpbd_2d::plugins::collision::{ Collider, Sensor };

use crate::{
    general::{ cursor_pos::MyWorldCoords, damage_timer::DamageTimer },
    projectiles::{ construct_projectile_from_attack_type, Projectile, ProjectileCollider, ProjectileEffect },
};

use super::Player;

#[derive(Component)]
pub struct AttackTimer(pub Timer);

#[derive(Clone, Copy)]
pub enum AttackType {
    Primary,
    Secondary
}

#[derive(Event)]
pub struct AttackEvent(Vec2, AttackType);


pub fn handle_attack_pressed(
    input: Res<ButtonInput<MouseButton>>,
    mut ew_attack: EventWriter<AttackEvent>,
    coords: ResMut<MyWorldCoords>
) {
    if input.pressed(MouseButton::Left) {
        ew_attack.send(AttackEvent(coords.0, AttackType::Primary));
    }
    if input.pressed(MouseButton::Right) {
        ew_attack.send(AttackEvent(coords.0, AttackType::Secondary));
    }
}

pub fn player_attack(
    mut commands: Commands,
    mut er_attack: EventReader<AttackEvent>,
    mut pl_q: Query<(&Player, &mut DamageTimer, &Transform)>
) {
    let (player, mut timer, transform) = pl_q.single_mut();

    for _event in er_attack.read() {
        if !timer.0.finished() {
            return;
        }
        timer.0.reset();
        let direction = vec3(
            _event.0.x - transform.translation.x,
            _event.0.y - transform.translation.y,
            0.0
        ).normalize();
        let bundle = construct_projectile_from_attack_type(_event.1, direction, transform);
        commands.spawn(bundle);
    }
}