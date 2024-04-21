use bevy_xpbd_2d::{ components::{ LinearVelocity, Position }, math::AdjustPrecision };

use crate::*;
use super::Player;

pub fn handle_movement(
    mut _commands: Commands,
    mut query: Query<(&mut LinearVelocity, &mut Position, &mut Player, &mut Sprite)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {
    for (mut vel, mut pos, mut char, mut sprite) in query.iter_mut() {
        let direction = get_direction(input.clone());

        //none of the buttons are pressed
        if direction == Vec3::ZERO {
            for (_, _, mut char, _) in &mut query {
                char.is_moving = false;
            }
            return;
        }
        pos.x += direction.normalize().x * char.speed * time.delta_seconds_f64().adjust_precision();
        pos.y += direction.normalize().y * char.speed * time.delta_seconds_f64().adjust_precision();
        vel.x = 0.;
        vel.y = 0.;
        //TODO: REWRITE TO NORMAL
        if direction.x < 0.0 {
            sprite.flip_x = true;
        } else if direction.x > 0.0 {
            sprite.flip_x = false;
        }

        char.is_moving = true;
    }
}

fn get_direction(input: ButtonInput<KeyCode>) -> Vec3 {
    let mut direction = Vec3::ZERO;

    if input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    direction
}
