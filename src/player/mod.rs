use bevy::prelude::{App, Plugin, StartupStage};

use self::player::{create_player, player_movement};
pub mod player;
pub mod structs;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, create_player)
            .add_system(player_movement);
    }
}
