use bevy::{prelude::*, reflect::TypeRegistry};
use bevy_rapier2d::{
    na::Vector2, physics::RapierConfiguration, rapier::dynamics::RigidBodyBuilder,
};
use rand::prelude::*;
use rapier2d::geometry::ColliderBuilder;
use serde::Deserialize;
use serde_json::Value;
use std::f32::consts::PI;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};

use super::algorithms::random_outline::room_outline;
use super::constants::*;

pub struct Ground;

pub struct SaveGame {
    pub pending: bool,
}

#[derive(Deserialize)]
struct Layer {
    data: Vec<usize>,
}

#[derive(Deserialize)]
struct Map {
    width: usize,
    height: usize,
    layers: Vec<Layer>,
}

pub fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut rapier_config: ResMut<RapierConfiguration>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    rapier_config.gravity = Vector2::zeros();
    // let mut file = File::open("./assets/map.json").expect("File open failed.");
    // let mut buffer = String::new();
    // file.read_to_string(&mut buffer).expect("File read failed.");
    // println!("{}", buffer);

    let file = File::open("./assets/room_1.json").expect("File open failed.");
    let reader = BufReader::new(file);

    let mini_map: Map = serde_json::from_reader(reader).expect("JSON deserialization failed.");
    let map_width = mini_map.width;
    let map_height = mini_map.height;
    let tiles = &mini_map.layers[0].data;

    let mut paths = HashMap::new();
    paths.insert(1, "0.png");
    paths.insert(2, "1.png");
    paths.insert(3, "2.png");
    paths.insert(4, "3.png");
    paths.insert(5, "4.png");
    paths.insert(6, "5.png");
    paths.insert(17, "16.png");
    paths.insert(18, "17.png");
    paths.insert(19, "18.png");
    paths.insert(20, "19.png");
    paths.insert(21, "20.png");
    paths.insert(33, "32.png");
    paths.insert(34, "33.png");
    paths.insert(35, "34.png");
    paths.insert(36, "35.png");
    paths.insert(37, "36.png");

    // for (i, value) in tiles.iter().enumerate() {
    //     if value != &18 && value != &37 {
    //         commands
    //             .spawn_bundle(SpriteBundle {
    //                 material: materials.add(asset_server.load(paths[value]).into()),
    //                 sprite: Sprite::new(Vec2::new(SIZE_32, SIZE_32)),
    //                 ..Default::default()
    //             })
    //             .insert(RigidBodyBuilder::new_static().translation(
    //                 (i as usize % map_width) as f32 * SIZE_32 - map_width as f32 / 2.0 * SIZE_32,
    //                 (map_height - i as usize / map_width) as f32 * SIZE_32
    //                     - map_height as f32 / 2.0 * SIZE_32,
    //             ))
    //             .insert(ColliderBuilder::cuboid(SIZE_32_PHYSICS, SIZE_32_PHYSICS).friction(0.0))
    //             .insert(Ground);
    //     } else {
    //         commands
    //             .spawn_bundle(SpriteBundle {
    //                 material: materials.add(asset_server.load(paths[value]).into()),
    //                 sprite: Sprite::new(Vec2::new(SIZE_32, SIZE_32)),
    //                 ..Default::default()
    //             })
    //             .insert(RigidBodyBuilder::new_static().translation(
    //                 (i as usize % map_width) as f32 * SIZE_32 - map_width as f32 / 2.0 * SIZE_32,
    //                 (map_height - i as usize / map_width) as f32 * SIZE_32
    //                     - map_height as f32 / 2.0 * SIZE_32,
    //             ))
    //             .insert(Ground);
    //     }
    // }

    // let room = room_outline(7, 9.0, 15.0);
    let room = room_outline(12, 9.0, 15.0);
    println!("{:?}", room);
    for (x, y, value) in room.iter() {
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(asset_server.load(paths[value]).into()),
                sprite: Sprite::new(Vec2::new(SIZE_32, SIZE_32)),
                ..Default::default()
            })
            .insert(
                RigidBodyBuilder::new_static()
                    .translation(*x as f32 * SIZE_32, *y as f32 * SIZE_32),
            )
            .insert(ColliderBuilder::cuboid(SIZE_32_PHYSICS, SIZE_32_PHYSICS).friction(0.0))
            .insert(Ground);
    }

    // let ground_blocks = 16;

    // for i in 0..ground_blocks {
    //     commands
    //         .spawn_bundle(SpriteBundle {
    //             material: materials.add(asset_server.load("ground_block.png").into()),
    //             sprite: Sprite::new(Vec2::new(SIZE_32, SIZE_32)),
    //             ..Default::default()
    //         })
    //         .with(RigidBodyBuilder::new_static().translation(
    //             i as f32 * SIZE_32 - ground_blocks as f32 / 2.0 * SIZE_32,
    //             -4.0 * SIZE_32,
    //         ))
    //         .with(ColliderBuilder::cuboid(SIZE_32_PHYSICS, SIZE_32_PHYSICS).friction(0.0))
    //         .with(Ground);
    // }

    let horizontal_tile_width = 62.0;
    let vertical_tile_width = 36.0;

    // top border
    // commands
    //     .spawn_bundle(SpriteBundle {
    //         material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
    //         sprite: Sprite::new(Vec2::new(horizontal_tile_width * SIZE_16, SIZE_16)),
    //         ..Default::default()
    //     })
    //     .insert(RigidBodyBuilder::new_static().translation(0.0, 296.0))
    //     .insert(ColliderBuilder::cuboid(
    //         horizontal_tile_width * SIZE_16_PHYSICS,
    //         SIZE_16_PHYSICS,
    //     ));

    // bottom border
    // commands
    //     .spawn_bundle(SpriteBundle {
    //         material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
    //         sprite: Sprite::new(Vec2::new(horizontal_tile_width * SIZE_16, SIZE_16)),
    //         ..Default::default()
    //     })
    //     .insert(RigidBodyBuilder::new_static().translation(0.0, -296.0))
    //     .insert(ColliderBuilder::cuboid(
    //         horizontal_tile_width * SIZE_16_PHYSICS,
    //         SIZE_16_PHYSICS,
    //     ));

    // left border
    // commands
    //     .spawn_bundle(SpriteBundle {
    //         material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
    //         sprite: Sprite::new(Vec2::new(SIZE_16, vertical_tile_width * SIZE_16)),
    //         ..Default::default()
    //     })
    //     .insert(RigidBodyBuilder::new_static().translation(-488.0, 0.0))
    //     .insert(ColliderBuilder::cuboid(
    //         SIZE_16_PHYSICS,
    //         vertical_tile_width * SIZE_16_PHYSICS,
    //     ));

    // right border
    // commands
    //     .spawn_bundle(SpriteBundle {
    //         material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
    //         sprite: Sprite::new(Vec2::new(SIZE_16, vertical_tile_width * SIZE_16)),
    //         ..Default::default()
    //     })
    //     .insert(RigidBodyBuilder::new_static().translation(488.0, 0.0))
    //     .insert(ColliderBuilder::cuboid(
    //         SIZE_16_PHYSICS,
    //         vertical_tile_width * SIZE_16_PHYSICS,
    //     ));
    // let body = RigidBodyBuilder::new_static().translation(0.0, -64.0);
    // let collider = ColliderBuilder::cuboid(16.0 * SIZE_16_PHYSICS, SIZE_16_PHYSICS);
    // commands.spawn((body, collider));

    // commands
    //     .spawn(SpriteBundle {
    //         material: materials.add(asset_server.load("ball.png").into()),
    //         sprite: Sprite::new(Vec2::new(10.0 * 32.0, 32.0)),
    //         ..Default::default()
    //     })
    //     .with(RigidBodyBuilder::new_static().translation(100.0, -64.0))
    //     .with(ColliderBuilder::cuboid(32.0, 256.0))
    //     .with(Ground);
}

// pub fn save(_world: &mut World, resources: &mut Resources) {
//     let mut save_game = resources.get_mut::<SaveGame>().unwrap();

//     if save_game.pending {
//         // let type_registry = resources.get::<TypeRegistry>().unwrap();
//         // let scene = DynamicScene::from_world(_world, &type_registry);

//         // println!("save");
//         // println!("{}", scene.serialize_ron(&type_registry).unwrap());
//         save_game.pending = false;
//     }
// }
