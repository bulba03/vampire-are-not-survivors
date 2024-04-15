use bevy::{ecs::{component::Component, query::{With, Without}, system::{Commands, Query, Res}}, log::info, prelude::{Deref, DerefMut}, time::{Time, Timer}, transform::components::Transform};

use crate::player::{Health, Player};

use super::Monster;

const PLAYER_DAMAGABLE_RADIUS: f32 = 20.;

pub fn damage_player(
    mut _commands: Commands,
    mut monsters_q: Query<(&mut Transform, &Monster, &mut DamageTimer)>,
    mut player_q: Query<(&Transform, &mut Health), (Without<Monster>, With<Player>)>,
    time: Res<Time>,
)   {
    let (pl_transform, mut health) = player_q.single_mut();
    for (transform, monster, mut timer) in monsters_q.iter_mut() {
        if pl_transform.translation.distance(transform.translation) <= PLAYER_DAMAGABLE_RADIUS {
            if timer.finished() {
                health.current -= monster.damage;
                timer.reset();
                
                info!("{}", health.current);
            }
            timer.tick(time.delta());
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct DamageTimer(pub Timer);