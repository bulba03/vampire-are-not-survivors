use bevy::time::{ Time, Timer };
use bevy::reflect::Reflect;
use bevy::prelude::{ Deref, DerefMut };
use bevy::ecs::{
    component::Component,
    entity::Entity,
    query::With,
    system::{ Commands, Query, Res },
};
use bevy_xpbd_2d::plugins::collision::CollidingEntities;

use crate::player::{ Health, Player };

use super::Monster;


pub fn handle_damage_timer(
    mut _commands: Commands,
    mut monsters_q: Query<&mut DamageTimer, With<Monster>>,
    time: Res<Time>
) {
    for mut timer in monsters_q.iter_mut() {
        timer.tick(time.delta());
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct DamageTimer(pub Timer);

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
                    if !health.deal_damage(monster.damage) {
                        _commands.entity(ent).despawn();
                    }
                }
            }
        }
    }
}
