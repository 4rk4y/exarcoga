use bevy::prelude::*;

use super::player::PlayerData;

pub struct Camera;

pub fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(Camera);
    commands.spawn_bundle(UiCameraBundle::default());
}

pub fn movement(player_data: Res<PlayerData>, mut query: Query<&mut Transform, With<Camera>>) {
    for mut transform in query.iter_mut() {
        transform.translation.x = player_data.position.x;
        transform.translation.y = player_data.position.y;
    }
}
