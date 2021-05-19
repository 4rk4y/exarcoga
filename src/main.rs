use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    sprite::SpriteSettings,
};
use bevy_rapier2d::physics::RapierPhysicsPlugin;

mod camera;
mod constants;
mod info;
mod level;
mod player;
mod algorithms {
    pub mod random_outline;
    pub mod rectangular_area;
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum GameState {
    Active,
    LoadRoom,
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            width: 1000.0,
            height: 620.0,
            resizable: false,
            title: String::from("Exarcoga - Exponential Area Resource Collecting Game"),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(
            223.0 / 255.0,
            223.0 / 255.0,
            223.0 / 255.0,
        )))
        .insert_resource(level::SaveGame { pending: false })
        .insert_resource(level::NextRoomData { room_number: 0 })
        .insert_resource(player::PlayerData {
            energy: 20.0,
            position: player::Position { x: 0.0, y: 0.0 },
        })
        .insert_resource(SpriteSettings {
            frustum_culling_enabled: true,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(LogDiagnosticsPlugin::default())
        .add_state(GameState::Active)
        .add_startup_system(camera::setup.system())
        .add_system(camera::movement.system().after("movement_player"))
        .add_startup_system(info::setup.system())
        .add_system(info::show.system())
        .add_startup_system(level::setup.system())
        // .add_system(level::save.system())
        .add_startup_system(player::setup.system())
        .add_system(player::movement.system().label("movement_player"))
        .add_system_set(
            SystemSet::on_enter(GameState::LoadRoom).with_system(level::despawn.system()),
        )
        .add_system_set(SystemSet::on_exit(GameState::LoadRoom).with_system(level::setup.system()))
        .run();
}
