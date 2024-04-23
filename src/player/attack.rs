use bevy::{
    ecs::{
        component::Component,
        event::{ Event, EventReader, EventWriter },
        system::{ Commands, Query, Res, ResMut },
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
    projectiles::{ Projectile, ProjectileCollider, ProjectileEffect },
};

use super::Player;

#[derive(Component)]
pub struct AttackTimer(pub Timer);

#[derive(Event)]
pub struct AttackEvent(Vec2);

pub fn handle_attack_pressed(
    input: Res<ButtonInput<MouseButton>>,
    mut ew_attack: EventWriter<AttackEvent>,
    coords: ResMut<MyWorldCoords>
) {
    if input.pressed(MouseButton::Left) {
        ew_attack.send(AttackEvent(coords.0));
    }
}

pub fn debug_player_attack(
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
        commands.spawn((
            SpriteBundle {
                sprite: Sprite { color: Color::RED, ..Default::default() },
                transform: Transform {
                    translation: vec3(transform.translation.x, transform.translation.y, 1.0),
                    scale: Vec3::splat(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            Projectile {
                effect: ProjectileEffect::None,
                damage: player.base_damage,
                direction,
                speed: 500.0,
            },
            ProjectileCollider,
            Collider::rectangle(3.0, 3.0),
            Sensor,
        ));
        info!("ASD");
    }
}
