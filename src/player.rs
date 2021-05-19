use super::{
    constants::*,
    level::{NextRoomData, SaveGame},
    GameState,
};
use bevy::prelude::*;
use bevy_rapier2d::{
    na::{Isometry2, Translation2, Vector2},
    physics::RigidBodyHandleComponent,
    rapier::dynamics::{RigidBodyBuilder, RigidBodySet},
};
use rapier2d::geometry::ColliderBuilder;

pub struct Position {
    pub x: f32,
    pub y: f32,
}
pub struct PlayerData {
    pub energy: f32,
    pub position: Position,
}

pub struct Player;

pub fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(asset_server.load("35.png").into()),
            sprite: Sprite::new(Vec2::new(SIZE_32, SIZE_32)),
            ..Default::default()
        })
        .insert(RigidBodyBuilder::new_dynamic())
        .insert(ColliderBuilder::cuboid(SIZE_32_PHYSICS, SIZE_32_PHYSICS).friction(0.0))
        .insert(Player);
}

pub fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut save_game: ResMut<SaveGame>,
    mut next_room: ResMut<NextRoomData>,
    mut player_data: ResMut<PlayerData>,
    mut rigid_bodies: ResMut<RigidBodySet>,
    mut game_state: ResMut<State<GameState>>,
    mut query: Query<&RigidBodyHandleComponent, With<Player>>,
) {
    if keyboard_input.just_pressed(KeyCode::S) {
        save_game.pending = true;
    }

    if keyboard_input.just_pressed(KeyCode::N) {
        next_room.room_number = 0;
        game_state.set(GameState::LoadRoom).unwrap();
    }

    for rigid_body_component in query.iter_mut() {
        let x_direction = if keyboard_input.pressed(KeyCode::A) {
            -1.0
        } else if keyboard_input.pressed(KeyCode::D) {
            1.0
        } else {
            0.0
        };

        let y_direction = if keyboard_input.pressed(KeyCode::W) {
            1.0
        } else if keyboard_input.pressed(KeyCode::S) {
            -1.0
        } else {
            0.0
        };

        if keyboard_input.just_pressed(KeyCode::P) {
            println!("hello");
            player_data.energy += 5.0;
        }

        let delta = time.delta_seconds() * 150.0;
        let direction = Translation2::new(x_direction * delta, y_direction * delta);

        let angular_velocity = Vector2::new(x_direction, y_direction) * delta;

        if let Some(rigid_body) = rigid_bodies.get_mut(rigid_body_component.handle()) {
            if direction.vector != Vector2::zeros() {
                rigid_body.set_linvel(angular_velocity * 50.0, true);
                // rigid_body.apply_impulse(angular_velocity * 300000.0, true);
                // let translation = Translation2::new(1.0, 1.0);
                // rigid_body.position().append_translation_mut(&translation);

                // set position
                // let position = rigid_body.position();
                // let position_translation =
                //     Vector2::new(position.translation.x, position.translation.y);
                // let mut new_position =
                //     Isometry2::new(position_translation, position.rotation.angle());
                // new_position.append_translation_mut(&direction);
                // rigid_body.set_position(new_position, true);

                player_data.energy -= 1.0;
            } else {
                rigid_body.set_linvel(Vector2::zeros(), true);
            }
            // To prevent engine from modifying angle set it to zero
            let position = rigid_body.position();
            let position_translation = Vector2::new(position.translation.x, position.translation.y);
            let new_position = Isometry2::new(position_translation, 0.0);
            rigid_body.set_position(new_position, true);
            rigid_body.set_angvel(0.0, true);

            player_data.position.x = new_position.translation.x;
            player_data.position.y = new_position.translation.y;
        }
    }
}
