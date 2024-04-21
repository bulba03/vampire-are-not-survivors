use bevy::math::Vec2;
use bevy::time::Timer;
use bevy::sprite::TextureAtlasLayout;
use bevy::render::texture::Image;
use bevy::ecs::system::{ Res, Resource };
use bevy::asset::{ AssetServer, Assets, Handle };

use crate::animation::AnimationIndices;

use super::monster_type::MonsterType;

const BAT_ANIM: &str = "monster/Monsters_Creatures_Fantasy/Flying eye/Flight.png";
const MUSHROOM_ANIM: &str = "monster/Monsters_Creatures_Fantasy/Mushroom/Run.png";
#[derive(Resource)]
pub struct MonsterCounter {
    pub(crate) enemy_count: i64,
    pub(crate) max_enemy_count: i64,
    pub(crate) enemy_spawn_timer: Timer,
}

#[derive(Resource)]
pub struct MonstersData {
    pub bat: MonsterSpriteData,
    pub mushroom: MonsterSpriteData,
}

impl MonstersData {
    pub fn construct(
        asset_server: Res<AssetServer>,
        texture_atlas_layouts: &mut Assets<TextureAtlasLayout>
    ) -> Self {
        let bat = MonsterSpriteData::construct_from_type(
            MonsterType::Bat,
            asset_server.clone(),
            texture_atlas_layouts
        );
        let mushroom = MonsterSpriteData::construct_from_type(
            MonsterType::Mushroom,
            asset_server.clone(),
            texture_atlas_layouts
        );
        MonstersData {
            bat,
            mushroom,
        }
    }
}

pub struct MonsterSpriteData {
    pub texture: Handle<Image>,
    pub atlas_layout: Handle<TextureAtlasLayout>,
    pub animation_indices: AnimationIndices,
}

impl MonsterSpriteData {
    pub fn construct_from_type(
        m_type: MonsterType,
        asset_server: AssetServer,
        texture_atlas_layouts: &mut Assets<TextureAtlasLayout>
    ) -> Self {
        match m_type {
            MonsterType::Bat => bat_sprite_data(asset_server.clone(), texture_atlas_layouts),
            MonsterType::Mushroom =>
                mushroom_sprite_data(asset_server.clone(), texture_atlas_layouts),
        }
    }
}

fn bat_sprite_data(
    asset_server: AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>
) -> MonsterSpriteData {
    let texture = asset_server.load(BAT_ANIM);
    let layout = TextureAtlasLayout::from_grid(Vec2::new(150.0, 150.0), 8, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 7 };
    MonsterSpriteData {
        texture,
        atlas_layout: texture_atlas_layout,
        animation_indices,
    }
}

fn mushroom_sprite_data(
    asset_server: AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>
) -> MonsterSpriteData {
    let texture = asset_server.load(MUSHROOM_ANIM);
    let layout = TextureAtlasLayout::from_grid(Vec2::new(150.0, 150.0), 8, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 7 };
    MonsterSpriteData {
        texture,
        atlas_layout: texture_atlas_layout,
        animation_indices,
    }
}
