use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity {
    pub v: Vec2,
}
#[derive(Component)]
pub struct Acceleration {
    pub a: Vec2,
}

#[derive(Component)]
pub struct Player{
    pub acc: Acceleration,
    pub vel: Velocity,
}

