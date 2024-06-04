use bevy::{asset::{AssetServer, Assets, Handle}, ecs::{component::Component, system::{Commands, Res, ResMut, Resource}}, math::Vec2, prelude::default, reflect::Reflect, render::{render_resource::Texture, texture::{self, Image}}, sprite::TextureAtlasLayout, utils::{HashMap, HashSet}};

use crate::animation::AnimationIndices;


#[derive(Resource)]
pub struct SheetsHolder {
    pub up: Handle<Image>,
    pub up_texture_layout: Handle<TextureAtlasLayout>,
    pub up_indices: AnimationIndices,
    pub down: Handle<Image>,
    pub down_texture_atlas_layout: Handle<TextureAtlasLayout>,
    pub down_indices: AnimationIndices,
    // attack: Handle<Image>,
}



impl SheetsHolder {
    pub fn construct_player(mut commands: Commands, asset_server: Res<AssetServer>,
        mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>)  {

            let texture1 = asset_server.load("rgl.png".to_string());
            let layout = TextureAtlasLayout::from_grid(Vec2::new(224.8, 256.0), 9, 1, None, None);
            let texture_atlas_layout = texture_atlas_layouts.add(layout);
            let animation_indices = AnimationIndices { first: 0, last: 8 };
            let texture2 = asset_server.load("rgl.png".to_string());
            commands.insert_resource(Self {
                up: texture1,
                up_indices: AnimationIndices { first: 0, last: 0 },
                up_texture_layout: texture_atlas_layout.clone(),
                down: texture2,
                down_indices: AnimationIndices { first: 0, last: 0 },
                down_texture_atlas_layout: texture_atlas_layout
                // attack: todo!(),
            });
        
    }
}

fn load_player_sprites(path: String, asset_server: &Res<AssetServer>,mut  texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) -> Handle<Image> {
    let texture = asset_server.load(path);
    let layout = TextureAtlasLayout::from_grid(Vec2::new(256.0, 512.0), 4, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 0 };
    return texture;
}