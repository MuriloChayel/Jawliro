use bevy::prelude::Component;

#[derive(Component)]
pub struct Player{
    vel: Velocity,
    acc: Acceleration
} 