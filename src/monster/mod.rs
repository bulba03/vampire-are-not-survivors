use bevy::transform::components::Transform;
use bevy::time::{Time, Timer, TimerMode};
use bevy::sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasLayout};
use bevy::prelude::default;
use bevy::math::{Vec2, Vec3Swizzles};
use bevy::ecs::{component::Component, query::With, system::{Commands, Query, Res, ResMut}};
use bevy::asset::{AssetServer, Assets};
use bevy::app::{Plugin, Startup, Update};
use rand::Rng;
mod movement;
mod monster_type;
mod resources;
mod damage;
use crate::player::Health;
use crate::{animation::{AnimationIndices, AnimationTimer}, player::Player};

use self::monster_type::MonsterType;
use self::damage::{damage_player, DamageTimer};
use self::resources::{MonsterCounter, MonstersData};

const ENEMY_SPAWN_RADIUS: f32 = 400.;
pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(
            MonsterCounter {enemy_count: 0,
                            max_enemy_count: 50,
                            enemy_spawn_timer: Timer::from_seconds(0.5, TimerMode::Repeating)
                        }
                    )
        .add_systems(Startup, load_monster_sprites)
        .add_systems(Update, (handle_enemy_spawn_timer, movement::move_to_player, damage_player));
    }
}

fn load_monster_sprites(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {
    commands.insert_resource(MonstersData::construct(asset_server, &mut texture_atlas_layouts));
}


#[derive(Component)]
pub struct Monster {
    pub hp: f32,
    pub damage: f32,
    pub speed: f32
}

fn handle_enemy_spawn_timer(
        time: Res<Time>, 
        mut counter: ResMut<MonsterCounter>,
        commands: Commands,
        player_q: Query<&Transform, With<Player>>,
        sprite_data: ResMut<MonstersData>)
    {
        if counter.enemy_count >= counter.max_enemy_count {
            return;
        }
        counter.enemy_spawn_timer.tick(time.delta());
        if counter.enemy_spawn_timer.just_finished() {
            let player = player_q.single();
            spawn_monster(commands, sprite_data,player);
            counter.enemy_count +=1;
        }
}

fn spawn_monster(mut commands: Commands,
                sprite_data: ResMut<MonstersData>,
                player: &Transform) {
                    let bat_data = &sprite_data.mushroom;
                    let texture = bat_data.texture.clone();
                    let texture_atlas_layout = bat_data.atlas_layout.clone();
                    let animation_indices = &bat_data.animation_indices;

                    let mut rng = rand::thread_rng();

                    let angle: f32 = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
                    let offset = Vec2::new(angle.cos(), angle.sin()) * ENEMY_SPAWN_RADIUS;
                    let enemy_position = player.translation.xy() + offset;

                    let mut sprite_bundle = SpriteSheetBundle{
                        texture,
                        atlas: TextureAtlas {
                            layout: texture_atlas_layout,
                            index: animation_indices.first,
                        },

                        transform: Transform::from_xyz(enemy_position.x, enemy_position.y, 1.),
                        ..default()
                    };
                    sprite_bundle.sprite.flip_x = enemy_position.x < player.translation.x;
                    let monster = Monster::construct_from_type(MonsterType::Bat);

                    commands.spawn(
                        (   
                            sprite_bundle,
                            AnimationIndices{ first: animation_indices.first, last: animation_indices.last },
                            Health {max: monster.hp, current: monster.hp},
                            monster,
                            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                            DamageTimer(Timer::from_seconds(0.5, TimerMode::Once)),
                           
                        )
                    );
}