use bevy::{app::{Plugin, Update}, ecs::{component::Component, system::{Query, Res}}, prelude::{Deref, DerefMut}, sprite::TextureAtlas, time::{Time, Timer}};
use crate::player::Player;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, animate_sprite);
    }
}

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas, Option<&Player>)>,
) {
    for (indices, mut timer, mut atlas, player) in &mut query {
        
        if let Some(player) = player {
            if !player.is_moving {
                continue;
            }
        }

        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}