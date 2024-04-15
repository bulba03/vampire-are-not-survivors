use bevy::{ecs::{query::{With, Without}, system::{Commands, Query, Res}}, sprite::Sprite, time::Time, transform::components::Transform};
use crate::player::Player;

use super::Monster;

pub fn move_to_player(
    mut _commands: Commands,
    mut monters_q: Query<(&mut Transform, &Monster, &mut Sprite)>,
    player_q: Query<&Transform, (With<Player>, Without<Monster>)>,
    time: Res<Time>
    ) {
        let player = player_q.single();
        for (mut transform, monster, mut sprite) in monters_q.iter_mut() {
            if player.translation.distance(transform.translation) <= 0.5 {
                continue;
            }
            let direction = (player.translation - transform.translation).normalize();
            transform.translation += direction * monster.speed * time.delta_seconds();
            sprite.flip_x = direction.x < 0.;
        }
}