use bevy::{ecs::{component::Component, entity::Entity, query::With, system::{Commands, Query, Res}}, prelude::{Deref, DerefMut}, reflect::Reflect, render::color::Color, sprite::Sprite, time::{Time, Timer}};
use bevy_xpbd_2d::plugins::collision::CollidingEntities;

use crate::player::{Health, Player};

use super::Monster;

// const PLAYER_DAMAGABLE_RADIUS: f32 = 20.;

pub fn handle_damage_timer(
    mut _commands: Commands,
    mut monsters_q: Query<&mut DamageTimer, With<Monster>>,
    // mut player_q: Query<(&Transform, &mut Health), (Without<Monster>, With<Player>)>,
    time: Res<Time>,
)   {
    for mut timer in monsters_q.iter_mut() {
        timer.tick(time.delta());
        
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct DamageTimer(pub Timer);

#[derive(Component, Default, Reflect)]
pub struct TestCol;

pub fn deal_damage_to_player(
    mut _commands: Commands,
    mut query: Query<(&mut Sprite, &mut Monster, &mut DamageTimer, &CollidingEntities), With<TestCol>>,
    mut p_q: Query<(Entity, &mut Health), With<Player>>
) {
    let (ent, mut health) = p_q.single_mut();
    for (mut sprite, monster, mut timer, colliding_entities) in &mut query {
        if colliding_entities.0.is_empty() {
            sprite.color = Color::BLUE;
        } else {
            if colliding_entities.get(&ent).is_some() {
                if timer.0.finished() {
                    timer.reset();
                    health.deal_damage(monster.damage);
                }
            }
            
            sprite.color = Color::RED;
            // info!("COLLISION");
        }

    }
}