use bevy::{app::{Plugin, PostUpdate, Startup, Update}, ecs::schedule::{IntoSystemConfigs, SystemSet}};

use crate::run_if_player_alive;

use self::{cursor_pos::{my_cursor_system, MyWorldCoords}, damage_timer::handle_damage_timer, health::update_health_bar, sheets_holder::SheetsHolder};

pub mod damage_timer;
pub mod cursor_pos;
pub mod health;
pub mod sheets_holder;


#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GeneralSet;

pub struct GeneralSystemsPlugin;

impl Plugin for GeneralSystemsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .add_systems(Update, my_cursor_system)
        .init_resource::<MyWorldCoords>()
        .add_systems(Startup, SheetsHolder::construct_player)
        .add_systems(Update, handle_damage_timer.in_set(GeneralSet))
        .add_systems(PostUpdate, update_health_bar.run_if(run_if_player_alive));
    }
}