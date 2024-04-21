use bevy::ecs::schedule::IntoSystemConfigs;
use bevy::transform::components::Transform;
use bevy::time::{ Time, Timer, TimerMode };
use bevy::sprite::TextureAtlasLayout;
use bevy::math::{ Vec2, Vec3Swizzles };
use bevy::ecs::{ component::Component, query::With, system::{ Commands, Query, Res, ResMut } };
use bevy::asset::{ AssetServer, Assets };
use bevy::app::{ Plugin, Startup, Update };
use rand::Rng;
mod movement;
mod monster_type;
mod resources;
mod bundles;
pub mod damage;
use crate::player::Player;
use crate::run_if_player_alive;

use self::bundles::{ bat_bundle, mushroom_bundle };
use self::damage::{ deal_damage_to_player, handle_damage_timer };
use self::resources::{ MonsterCounter, MonstersData };

const ENEMY_SPAWN_RADIUS: f32 = 400.0;
pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(MonsterCounter {
            enemy_count: 0,
            max_enemy_count: 50,
            enemy_spawn_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        })
            .add_systems(Startup, load_monster_sprites)
            .add_systems(Update, (
                handle_enemy_spawn_timer,
                movement::move_to_player,
                handle_damage_timer,
                deal_damage_to_player,
            ).run_if(run_if_player_alive));
    }
}

fn load_monster_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    commands.insert_resource(MonstersData::construct(asset_server, &mut texture_atlas_layouts));
}

#[derive(Component)]
pub struct Monster {
    pub hp: f32,
    pub damage: f32,
    pub speed: f32,
}

fn handle_enemy_spawn_timer(
    time: Res<Time>,
    mut counter: ResMut<MonsterCounter>,
    commands: Commands,
    player_q: Query<&Transform, With<Player>>,
    sprite_data: ResMut<MonstersData>
) {
    if counter.enemy_count >= counter.max_enemy_count {
        return;
    }
    counter.enemy_spawn_timer.tick(time.delta());
    if counter.enemy_spawn_timer.just_finished() {
        let player = player_q.single();
        spawn_monster(commands, sprite_data, player);
        counter.enemy_count += 1;
    }
}

fn spawn_monster(mut commands: Commands, sprite_data: ResMut<MonstersData>, player: &Transform) {
    let monsters_data = sprite_data;
    let mut rng = rand::thread_rng();

    let angle: f32 = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
    let offset = Vec2::new(angle.cos(), angle.sin()) * ENEMY_SPAWN_RADIUS;
    let enemy_position = player.translation.xy() + offset;
    let flip_sprite = enemy_position.x < player.translation.x;
    let bundle = match rng.gen_range(0..=1) {
        0 => mushroom_bundle(&monsters_data, enemy_position, flip_sprite),
        1 => bat_bundle(&monsters_data, enemy_position, flip_sprite),
        _ => todo!(),
    };
    commands.spawn(bundle);
}
