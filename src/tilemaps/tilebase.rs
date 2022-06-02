use std::fmt::format;

use super::*;

pub fn create_world_grid(
    mut commands: Commands,
    tilemap: ResMut<Tilemap>
){
    // MATRIZ DO MUNDO (TILES)
    for y in 0..tilemap.height {
        for x in 0..tilemap.width {
            commands
                .spawn()
                .insert(Tile {
                    x: x,
                    y: y,
                    soma: 0,
                    material: 0,
                });
                // .insert_bundle(SpriteBundle {
                //     texture: server.load("tiles/tile_{}", ),
                //     transform: Transform {
                //         translation: Vec3::new(x as f32 * 16., y as f32 * 16., 0.),
                //         ..default()
                //     },
                //     ..default()
                // });
        }
    }
}
pub fn populate_grid(query: Query<&Tile, With<Tile>>, mut tilemap: ResMut<Tilemap>) {
    for tile in query.iter() {
        tilemap.storage.push(tile.clone());
    }
    //println!("tile {:?}",tilemap.storage.len());
}
pub fn update_tiles(
    mut tilemap: ResMut<Tilemap>
) {
    let mut j: u32 = 0;

    for i in 0..tilemap.storage.len() {
        //Lista tiles
        //DIREITA
        if i < tilemap.storage.len() {
            if !((i + 1) % tilemap.width as usize == 0) {
                if tilemap.storage[i + 1].material == 0 {
                    //proximo da direita
                    tilemap.storage[i].soma += 1;
                }
            }
            //BAIXO
            if i < (tilemap.width * (tilemap.height - 1)) as usize {
                if tilemap.storage[i + tilemap.width as usize].material == 0 {
                    tilemap.storage[i].soma += 2;
                }
            }
            //ESQUERDA
            if i > 0 && !(i % tilemap.width as usize == 0) {
                if tilemap.storage[i - 1 as usize].material == 0 {
                    tilemap.storage[i].soma += 4;
                }
            }
            //CIMA
            if i > (tilemap.width - 1) as usize {
                if tilemap.storage[i - tilemap.width as usize].material == 0 {
                    tilemap.storage[i].soma += 8;
                }
            }
        }

        println!("tile: {:?} , {:?}", tilemap.storage[i], j);
        j += 1;
    }
}
pub fn update(
    mut query: Query<Entity, With<Tile>>,
    mut commands: Commands,
    server: Res<AssetServer>,
    tilemap: ResMut<Tilemap>

){

    for (index, entidade) in query.iter_mut().enumerate(){
        println!("{:?}",index);
        
        let texture_index =  get_texture_index(tilemap.storage[index].soma);
        let path = format!("tiles/tile_{:?}.png", texture_index);
        let tile = tilemap.storage[index];

        println!("Soma: {:?}", tile.soma);
        println!("{}",path);

        commands.entity(entidade).insert_bundle(SpriteBundle {
                texture: server.load(&path),
                transform: Transform {
                    translation: Vec3::new(tile.x as f32 * 16., tile.y as f32 * 16., 0.),
                    ..default()
                },
                ..default()
            });
    }
}

pub fn get_texture_index(sum: u32) -> i32{
    let x = match sum{
        // soma -> index
        //(BOTTOM)
        3 =>    6,
        7 =>    7,
        6 =>    8,

        //(MIDDLE)
        11 =>   3,
        15 =>   4,
        14 =>   5,
        
        //(TOP)
        9 =>    0,
        13 =>   1,
        12 =>   2,

        //(EXEPTIONS HORIZONTAL)
        2 => 14,
        10 => 13,
        8 => 12,

        //(EXEPTIONS VERTICAL)
        4 => 11,
        5 => 10,
        1 => 9,

        0 => 15,
        


        _ =>    -1
    };
    println!("x: {:?}", x);
    return x;
}