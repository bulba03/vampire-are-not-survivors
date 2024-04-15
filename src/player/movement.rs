use crate::*;
use super::Player;

pub fn handle_movement( 
        mut _commands: Commands,
        mut query: Query<(&mut Transform, &mut Player, &mut Sprite)>,
        input: Res<ButtonInput<KeyCode>>,
        time: Res<Time>
        ) {
            let (mut transform, mut char, mut sprite) = query.single_mut();
            let direction = get_direction(input.clone());

            //none of the buttons are pressed
            if direction == Vec3::ZERO {
                for (_, mut char, _) in &mut query {
                    char.is_moving = false;
                }
                return;
            }
            
            transform.translation += direction.normalize() * char.speed* time.delta_seconds();
            
            //TODO: REWRITE TO NORMAL
            if direction.x < 0. {
                 sprite.flip_x = true;
            }
            else if direction.x > 0. {
                sprite.flip_x = false;
            }
                
            char.is_moving = true;
}

//TODO: Is it normal to pass Res<> ?
fn get_direction(input: ButtonInput<KeyCode>) -> Vec3 {
    let mut direction = Vec3::ZERO;
            
    if input.pressed(KeyCode::KeyA) {
        direction.x -=1.;
    }
    if input.pressed(KeyCode::KeyD) {
        direction.x +=1.;
    }
    if input.pressed(KeyCode::KeyW) {
        direction.y +=1.;
    }
    if input.pressed(KeyCode::KeyS) {
        direction.y -=1.;
    }
    direction
}