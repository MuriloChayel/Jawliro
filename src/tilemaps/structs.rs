use bevy::prelude::*;

// use std::slice::Iter;

// enum GroundTextures {
//     top_left_corner = 3,  // 0
//     top_middle = 7,       // 1
//     top_right_corner = 6, // 2
//     midle_left = 11,      // 3
//     center = 15,
//     middle_right = 14,
//     bottom_left_corner = 9,
//     bottom_middle = 13,
//     bottom_right_corner = 12,
// }
// impl GroundTextures {
//     pub fn iterator() -> Iter<'static, GroundTextures> {
//         static GROUND_TEXTURES: [GroundTextures; 9] = [
//             GroundTextures::top_left_corner,
//             GroundTextures::top_middle,
//             GroundTextures::top_right_corner,
//             GroundTextures::midle_left,
//             GroundTextures::center,
//             GroundTextures::middle_right,
//             GroundTextures::bottom_left_corner,
//             GroundTextures::bottom_middle,
//             GroundTextures::bottom_right_corner,
//         ];
//         GROUND_TEXTURES.iter()
//     }
// }

//base tile
#[derive(Component, Debug, Copy, Clone)]
pub struct Tile {
    pub x: i64,
    pub y: i64,
    pub soma: u32,
    pub material: u32,
}

#[derive(Component, Debug)]
pub struct Tilemap {
    pub storage: Vec<Tile>,
    pub width: i64,
    pub height: i64,
    pub pivot: (i64, i64),
}

#[derive(Component, Debug)]
pub struct Grid {
    pub width: i64,
    pub height: i64,
    pub pivot: (i64, i64),
}

#[derive(Component, Debug)]
pub struct Cell {
    pub x: i64,
    pub y: i64,
}
