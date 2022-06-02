use bevy::prelude::{Commands, OrthographicCameraBundle};

pub fn create_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale *= 0.3;

    commands.spawn_bundle(camera);
}
