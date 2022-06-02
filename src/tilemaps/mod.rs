use bevy::prelude::*;
use std::slice::Iter;
pub mod tilebase;

enum Ground_textures {
    top_left_corner = 3, // 0
    top_middle = 7,      // 1
    top_right_corner = 6,// 2
    midle_left = 11,     // 3  
    center = 15,
    middle_right = 14,
    bottom_left_corner = 9,
    bottom_middle = 13,
    bottom_right_corner = 12,
}
impl Ground_textures {
    pub fn iterator() -> Iter<'static, Ground_textures> {
        static GROUND_TEXTURES: [Ground_textures; 9] = [ 
        Ground_textures::top_left_corner, 
        Ground_textures::top_middle,
        Ground_textures::top_right_corner,
        Ground_textures::midle_left,
        Ground_textures::center,
        Ground_textures::middle_right,
        Ground_textures::bottom_left_corner,
        Ground_textures::bottom_middle,
        Ground_textures::bottom_right_corner,
        ];
        GROUND_TEXTURES.iter()
    }
}

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
