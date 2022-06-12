use bevy::prelude::*;


const PATH: &str = "player/liro.png";


pub fn create_player(
    mut commands: Commands,
    server: Res<AssetServer>
){
    commands.spawn_bundle(
        SpriteBundle{
            texture: server.load(&PATH),
            transform: Transform {
                translation: Vec3::new(0.,0.,0.),
                ..default()
            },
            ..default()
    })
    .insert(Player);
}