mod camera;
mod player;
mod preferences;
mod tilemaps;

use bevy::prelude::*;
use camera::ortho_camera::create_camera;
use player::PlayerPlugin;
use preferences::*;
use Jawliro::tilemaps::{
    structs::{Grid, Tilemap},
    tilebase::{create_world_grid, populate_grid, update, update_tiles},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            width: 1280.,
            height: 720.,
            title: "Game".to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(BACKGROUND))
        .insert_resource(Grid {
            width: 40,
            height: 40,
            pivot: (0, 0),
        })
        .insert_resource(Tilemap {
            storage: Vec::new(),
            width: 8,
            height: 5,
            pivot: (-10, -6),
        })
        .add_startup_system_to_stage(StartupStage::PreStartup, create_world_grid)
        .add_startup_system_to_stage(StartupStage::Startup, populate_grid)
        .add_startup_system_to_stage(StartupStage::PostStartup, update_tiles)
        .add_startup_system_to_stage(StartupStage::PostStartup, update.after(update_tiles))
        .add_plugin(PlayerPlugin)
        .add_startup_system(create_camera)
        //.add_system(show_tiles)
        .run();
}
