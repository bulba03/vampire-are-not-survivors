use bevy::{
    ecs::{ component::Component, system::{ Commands, Query, Res } },
    prelude::{ Deref, DerefMut },
    time::{ Time, Timer },
};

pub fn handle_damage_timer(
    mut _commands: Commands,
    mut monsters_q: Query<&mut DamageTimer>,
    time: Res<Time>
) {
    for mut timer in monsters_q.iter_mut() {
        timer.tick(time.delta());
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct DamageTimer(pub Timer);
