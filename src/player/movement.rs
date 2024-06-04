use bevy_xpbd_2d::{ components::{ LinearVelocity, Position }, math::AdjustPrecision };

use crate::*;
use self::general::sheets_holder::SheetsHolder;

use super::Player;

pub fn handle_movement(
    mut _commands: Commands,
    mut query: Query<(&mut LinearVelocity, &mut Position, &mut Player, &mut Sprite,&mut Handle<Image>)>,
    input: Res<ButtonInput<KeyCode>>,
    pl_sprites: Res<SheetsHolder>,
    time: Res<Time>
) {
    for (mut vel, mut pos, mut char, mut sprite, mut texture) in query.iter_mut() {
        let direction = get_direction(input.clone());

        //none of the buttons are pressed
        if direction == Vec3::ZERO {
            char.is_moving = false;
            return;
        }
        if direction.y < 0. {
            *texture = pl_sprites.down.clone();
        }
        if direction.y > 0. {
            *texture = pl_sprites.up.clone();
        }
        pos.x += direction.normalize().x * char.speed * time.delta_seconds_f64().adjust_precision();
        pos.y += direction.normalize().y * char.speed * time.delta_seconds_f64().adjust_precision();
        vel.x = 0.0;
        vel.y = 0.0;
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
