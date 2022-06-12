use bevy::prelude::{Component, Vec2};

#[derive(Component)]
pub struct Velocity{
    pub v: Vec2
}
#[derive(Component)]
pub struct Acceleration{
    pub a: Vec2
}