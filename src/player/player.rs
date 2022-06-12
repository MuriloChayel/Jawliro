use bevy::{sprite::SpriteBundle, prelude::{Commands, Res, AssetServer, Transform, default, KeyCode, With, Query}, math::{Vec3, Vec2}, input::Input};

use super::structs::{Player, Velocity, Acceleration};

const PATH: &str = "player/liro.png";

pub fn create_player(mut commands: Commands, server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: server.load(PATH),
            transform: Transform {
                translation: Vec3::new(0., 80., 0.),
                ..default()
            },
            ..default()
        })
        .insert(Player{
            acc: Acceleration { a: Vec2::new(0., 0.) },
            vel: Velocity { v: Vec2::new(5., 0.) },
        }
    );
}

pub fn player_movement(
    key_pressed: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &Player), With<Player>>,
){  
    for (mut player_transform, player) in player_query.iter_mut(){
        let player_translaction = &mut player_transform.translation;
        if key_pressed.pressed(KeyCode::A){
            player_translaction.x -= player.vel.v.x;
        }

        if key_pressed.pressed(KeyCode::D){
            player_translaction.x += player.vel.v.x;
        }

        if key_pressed.just_pressed(KeyCode::W){
            player_translaction.y += 6.;
        }

        if key_pressed.just_released(KeyCode::W){
            player_translaction.y -= 6.;
        }
    }

    
    
    


}

    