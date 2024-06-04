use bevy::{
    core::Name, ecs::bundle::Bundle, math::Vec2, prelude::default, sprite::{ Sprite, SpriteSheetBundle, TextureAtlas }, time::{ Timer, TimerMode }, transform::components::Transform
};
use bevy_xpbd_2d::{ components::{ LockedAxes, RigidBody }, plugins::collision::Collider };

use crate::animation::{ AnimationIndices, AnimationTimer };
use crate::general::health::Health;

use super::{ damage::MonsterCollider, monster_type::MonsterType, resources::MonstersData, Monster };
use crate::general::damage_timer::DamageTimer;

#[derive(Bundle)]
pub struct MonsterBundle {
    sprite_sheet_bundle: SpriteSheetBundle,
    health: Health,
    animation_indices: AnimationIndices,
    rigidbody: RigidBody,
    collider: Collider,
    locked_axis: LockedAxes,
    monster_collider: MonsterCollider,
    animation_timer: AnimationTimer,
    damage_timer: DamageTimer,
    monster: Monster,
    name: Name
}

pub fn mushroom_bundle(sprite_data: &MonstersData, pos: Vec2, flip_sprite: bool) -> MonsterBundle {
    let sprite_data = &sprite_data.mushroom;
    let texture = sprite_data.texture.clone();
    let texture_atlas_layout = sprite_data.atlas_layout.clone();
    let animation_indices = &sprite_data.animation_indices;
    let sprite = Sprite {
        flip_x: flip_sprite,
        ..default()
    };
    let mut sprite_sheet_bundle = SpriteSheetBundle {
        texture,
        atlas: TextureAtlas {
            layout: texture_atlas_layout,
            index: animation_indices.first,
        },
        sprite,
        transform: Transform::from_xyz(pos.x, pos.y, 1.0),

        ..default()
    };
    sprite_sheet_bundle.sprite.flip_x = flip_sprite;
    let monster = Monster::construct_from_type(MonsterType::Mushroom);

    MonsterBundle {
        sprite_sheet_bundle,
        health: Health { max: monster.hp, current: monster.hp },
        animation_indices: AnimationIndices {
            first: animation_indices.first,
            last: animation_indices.last,
        },
        rigidbody: RigidBody::Dynamic,
        collider: Collider::rectangle(20.0, 30.0),
        locked_axis: LockedAxes::ROTATION_LOCKED,
        monster_collider: MonsterCollider,
        animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        damage_timer: DamageTimer(Timer::from_seconds(0.5, TimerMode::Once)),
        monster,
        name: Name::new("Mushroom")
    }
}

pub fn bat_bundle(sprite_data: &MonstersData, pos: Vec2, flip_sprite: bool) -> MonsterBundle {
    let sprite_data = &sprite_data.bat;
    let texture = sprite_data.texture.clone();
    let texture_atlas_layout = sprite_data.atlas_layout.clone();
    let animation_indices = &sprite_data.animation_indices;
    let sprite = Sprite {
        flip_x: flip_sprite,
        ..default()
    };
    let mut sprite_sheet_bundle = SpriteSheetBundle {
        texture,
        atlas: TextureAtlas {
            layout: texture_atlas_layout,
            index: animation_indices.first,
        },
        sprite,
        transform: Transform::from_xyz(pos.x, pos.y, 1.0),

        ..default()
    };
    sprite_sheet_bundle.sprite.flip_x = flip_sprite;
    let monster = Monster::construct_from_type(MonsterType::Bat);

    MonsterBundle {
        sprite_sheet_bundle,
        health: Health { max: monster.hp, current: monster.hp },
        animation_indices: AnimationIndices {
            first: animation_indices.first,
            last: animation_indices.last,
        },
        rigidbody: RigidBody::Dynamic,
        collider: Collider::rectangle(20.0, 30.0),
        locked_axis: LockedAxes::ROTATION_LOCKED,
        monster_collider: MonsterCollider,
        animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        damage_timer: DamageTimer(Timer::from_seconds(0.5, TimerMode::Once)),
        monster,
        name: Name::new("Bat")
    }
}
