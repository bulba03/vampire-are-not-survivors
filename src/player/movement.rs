use crate::*;

use super::Player;
pub fn handle_movement(mut _commands: Commands,
    mut query: Query<(&mut Transform, &mut Player, &mut Sprite)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>) {
    let mut direction = Vec3::splat(0.);
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
    if direction == Vec3::ZERO {
        for (_, mut char, _) in &mut query {
            char.is_moving = false;
        }
        return;
    }
    for (mut transform, mut char, _) in &mut query {
        transform.translation += direction * char.speed* time.delta_seconds();
        char.is_moving = true;
    }
}