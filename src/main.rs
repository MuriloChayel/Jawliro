use preferences::*;
mod preferences;
mod Tilemaps;
use tilemaps::{
    tilebase::{create_world_grid, populate_grid, update_tiles, update},
    Grid, Tilemap,
};
mod tilemaps;

use camera::ortho_camera::create_camera;
mod camera;
use bevy::{prelude::*};
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
            width: 7,
            height: 2,
            pivot: (-10, -6),
        })
        .add_startup_system_to_stage(StartupStage::PreStartup, create_world_grid)
        .add_startup_system_to_stage(StartupStage::Startup, populate_grid)
        .add_startup_system_to_stage(StartupStage::PostStartup, update_tiles)
        .add_startup_system_to_stage(StartupStage::PostStartup, update.after(update_tiles))
        .add_startup_system(create_camera)
        //.add_system(show_tiles)
        .run();
}