use bevy::reflect::Reflect;
use bevy::ecs::{ component::Component, entity::Entity, query::With, system::{ Commands, Query } };
use bevy_xpbd_2d::plugins::collision::CollidingEntities;

use crate::general::damage_timer::DamageTimer;
use crate::player::Player;
use crate::general::health::Health;

use super::Monster;

#[derive(Component, Default, Reflect)]
pub struct MonsterCollider;

pub fn deal_damage_to_player(
    mut _commands: Commands,
    mut query: Query<(&mut Monster, &mut DamageTimer, &CollidingEntities), With<MonsterCollider>>,
    mut p_q: Query<(Entity, &mut Health), With<Player>>
) {
    let (ent, mut health) = p_q.get_single_mut().unwrap();
    for (monster, mut timer, colliding_entities) in &mut query {
        if colliding_entities.0.is_empty() {
            continue;
        } else {
            if colliding_entities.get(&ent).is_some() {
                if timer.0.finished() {
                    timer.reset();
                    health.deal_damage(monster.damage);
                    if health.is_alive() == false {
                        _commands.entity(ent).despawn();
                    }
                }
            }
        }
    }
}
