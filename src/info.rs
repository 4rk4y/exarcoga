use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub struct Energy;

use super::player::PlayerData;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position: Rect {
                    top: Val::Px(25.0),
                    left: Val::Px(25.0),
                    ..Default::default()
                },
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            // text: Text {
            //     font: asset_server.load("Nanum_Gothic_Coding/NanumGothicCoding-Regular.ttf"),
            //     style: TextStyle {
            //         color: Color::rgb(0.0, 0.0, 0.0),
            //         font_size: 32.0,
            //         ..Default::default()
            //     },
            //     value: "Test".to_string(),
            // },
            text: Text::with_section(
                "Test",
                TextStyle {
                    font: asset_server.load("Nanum_Gothic_Coding/NanumGothicCoding-Regular.ttf"),
                    color: Color::rgb(0.7, 0.7, 0.7),
                    font_size: 32.0,
                    ..Default::default()
                },
                TextAlignment::default(),
            ),
            ..Default::default()
        })
        .insert(Energy);
}

pub fn show(
    diagnostics: Res<Diagnostics>,
    player_data: Res<PlayerData>,
    mut query1: Query<&mut Text, With<Energy>>,
) {
    for mut text in query1.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_average) = fps.average() {
                // text.sections[0].value = format!(
                //     "FPS: {}\nEnergy: {}",
                //     fps_average.round(),
                //     player_data.energy
                // );
                text.sections[0].value = format!("FPS: {}", fps_average.round());
            }
        }
    }
}
