use bevy_rapier2d::na::Vector2;
use rand::prelude::*;
use std::f32::consts::PI;

pub fn room_outline(points: usize, min_radius: f32, max_radius: f32) -> Vec<(isize, isize, usize)> {
    // let mut tiles = Vec::new();
    // tiles.push((1, 1));

    let radii = (0..points)
        .into_iter()
        .map(|_| rand::thread_rng().gen_range(min_radius..max_radius));

    let angle = 2.0 * PI / points as f32;
    let angles = (0..points).into_iter().map(|point| point as f32 * angle);

    let points = radii
        .zip(angles)
        .map(|(radius, angle)| (radius * angle.cos(), radius * angle.sin()));

    // println!("{:?}", points.collect::<Vec<(f32, f32)>>());

    let mut points = points.collect::<Vec<(f32, f32)>>();
    // let Some(first_point) = points.first().unwrap::<(f32, f32)>();
    if let Some(first_point) = points.first() {
        points.push(((*first_point).0, (*first_point).1 + 0.2));
    }
    return tiles(points);

    // return tiles(points.collect::<Vec<(f32, f32)>>());

    // let mut fake_points = Vec::new();
    // fake_points.push((4.2, 0.0));
    // fake_points.push((1.0, 3.0));
    // fake_points.push((-3.42, 2.48));
    // fake_points.push((-3.99, -2.9));
    // fake_points.push((1.47, -4.53));
    // fake_points.push((4.2, 0.0));

    // return tiles(fake_points);
}

fn tiles(points: Vec<(f32, f32)>) -> Vec<(isize, isize, usize)> {
    let mut tiles = Vec::new();

    let mut points_iter = points.into_iter();
    let current_point = points_iter.next();
    // let next_point = points_iter.next();

    let mut side = 1;
    let mut tile_x = 0.0;
    let mut tile_y = 0.0;

    let mut current_point = current_point.unwrap();

    // if let Some(current_point) = current_point {
    //     tile_x = current_point.0.floor();
    //     tile_y = current_point.1;
    //     tiles.push((tile_x as usize, tile_y as usize));
    //     // TODO while loop
    //     // if (point_is_inside_tile(next_point, tile_x, tile_y)) {
    //     //     points_iter.next();
    //     // }
    // } else {
    //     panic!("Could not get first point.")
    // }

    tile_x = current_point.0.floor();
    tile_y = current_point.1;
    tiles.push((tile_x as isize, tile_y as isize, 19));

    println!("---{:?}", current_point);
    for next_point in points_iter {
        let mut counter = 0;
        let mut next_point_found = false;
        let p1 = Vector2::new(current_point.0, current_point.1);
        let p2 = Vector2::new(next_point.0, next_point.1);

        let v = Vector2::new(p2.x - p1.x, p2.y - p1.y);

        let m = v.y / v.x;

        let h = p1.y - p1.x * m;

        while !next_point_found && counter < 100 {
            println!(
                "side: {}, tile_x: {}, tile_y: {}, m: {}, h: {}",
                match side {
                    0 => "right",
                    1 => "top",
                    2 => "left",
                    3 => "bottom",
                    _ => "error",
                },
                tile_x,
                tile_y,
                m,
                h
            );

            let tile = next_tile(side, tile_x, tile_y, m, h, next_point.0, next_point.1);
            println!("--");
            tiles.push((
                tile_x as isize,
                tile_y as isize,
                match (side, tile.0) {
                    // (current, next) if (current, next) == (0, 0) => 19,
                    // (current, next) if (current, next) == (0, 1) => 3,
                    // (current, next) if (current, next) == (1, 1) => 2,
                    // (current, next) if (current, next) == (1, 2) => 1,
                    // (current, next) if (current, next) == (2, 2) => 17,
                    // (current, next) if (current, next) == (2, 3) => 33,
                    // (current, next) if (current, next) == (3, 3) => 34,
                    // (current, next) if (current, next) == (3, 0) => 35,
                    // (current, next) if (current, next) == (2, 1) => 4,
                    // (current, next) if (current, next) == (1, 0) => 5,
                    // (current, next) if (current, next) == (3, 2) => 20,
                    // (current, next) if (current, next) == (0, 3) => 21,
                    (current, next) if (current, next) == (0, 0) => 2,
                    (current, next) if (current, next) == (0, 1) => 35,
                    (current, next) if (current, next) == (1, 1) => 19,
                    (current, next) if (current, next) == (1, 2) => 3,
                    (current, next) if (current, next) == (2, 2) => 2,
                    (current, next) if (current, next) == (2, 3) => 1,
                    (current, next) if (current, next) == (3, 3) => 17,
                    (current, next) if (current, next) == (3, 0) => 33,
                    (current, next) if (current, next) == (2, 1) => 5,
                    (current, next) if (current, next) == (1, 0) => 21,
                    (current, next) if (current, next) == (3, 2) => 4,
                    (current, next) if (current, next) == (0, 3) => 20,
                    (current, next) => {
                        println!("{:?}", (current, next));
                        18
                    }
                },
            ));
            side = tile.0;
            tile_x = tile.1;
            tile_y = tile.2;

            if point_is_inside_tile(next_point, tile_x, tile_y) {
                next_point_found = true;
                current_point = (next_point.0, next_point.1);
            }
            counter += 1;
        }
        println!("---{:?}", next_point);
    }

    tiles
}

fn point_is_inside_tile(point: (f32, f32), tile_x: f32, tile_y: f32) -> bool {
    tile_x <= point.0 && point.0 <= tile_x + 1.0 && tile_y <= point.1 && point.1 <= tile_y + 1.0
}

fn next_tile(
    current_side: u8,
    tile_x: f32,
    tile_y: f32,
    m: f32,
    h: f32,
    next_point_x: f32,
    next_point_y: f32,
) -> (u8, f32, f32) {
    let side_0 = (tile_x + 1.0) * m + h;
    let side_1 = (tile_y + 1.0 - h) / m;
    let side_2 = (tile_x) * m + h;
    let side_3 = (tile_y - h) / m;

    println!(
        "right: {}, top: {}, left: {}, bottom: {}",
        side_0, side_1, side_2, side_3
    );

    let mut next_side = 5;
    let mut distance_to_next_point = 0.0;
    let mut side_found = false;

    let margin = 0.0001;

    if current_side != 2 && tile_y - margin <= side_0 && side_0 <= tile_y + 1.0 + margin {
        next_side = 0; // right
        distance_to_next_point =
            ((next_point_x - tile_x).powf(2.0) + (next_point_y - side_0).powf(2.0)).sqrt();
        side_found = true;
    }
    if current_side != 3 && tile_x - margin <= side_1 && side_1 <= tile_x + 1.0 + margin {
        let distance_to_next_point_temp =
            ((next_point_x - side_1).powf(2.0) + (next_point_y - tile_y).powf(2.0)).sqrt();

        if !side_found || distance_to_next_point_temp < distance_to_next_point {
            next_side = 1; // top
        }
        distance_to_next_point = distance_to_next_point_temp;
        side_found = true;
    }
    if current_side != 0 && tile_y - margin <= side_2 && side_2 <= tile_y + 1.0 + margin {
        let distance_to_next_point_temp =
            ((next_point_x - tile_x).powf(2.0) + (next_point_y - side_2).powf(2.0)).sqrt();

        if !side_found || distance_to_next_point_temp < distance_to_next_point {
            next_side = 2; // left
        }
        distance_to_next_point = distance_to_next_point_temp;
        side_found = true;
    }
    if current_side != 1 && tile_x - margin <= side_3 && side_3 <= tile_x + 1.0 + margin {
        let distance_to_next_point_temp =
            ((next_point_x - side_3).powf(2.0) + (next_point_y - tile_y).powf(2.0)).sqrt();

        println!(
            "distance_to_next_point_temp: {}, distance_to_next_point: {}",
            distance_to_next_point_temp, distance_to_next_point
        );
        if !side_found || distance_to_next_point_temp < distance_to_next_point {
            next_side = 3; // bottom
        }
        side_found = true;
    }

    match next_side {
        0 => return (next_side, tile_x + 1.0, tile_y),
        1 => return (next_side, tile_x, tile_y + 1.0),
        2 => return (next_side, tile_x - 1.0, tile_y),
        3 => return (next_side, tile_x, tile_y - 1.0),
        _ => panic!("Next side not found."),
    }
}

#[test]
fn room_outline_test() {
    let result = room_outline(5, 3.0, 5.0);
    println!("{:?}", result)
}

#[test]
fn vector_test() {
    let x1 = Vector2::new(1.5, 1.0);
    let x2 = Vector2::new(3.5, 2.5);

    let x3 = Vector2::new(1.0, 0.0);
    let x4 = Vector2::new(2.0, 0.0);

    let v1 = Vector2::new(x2.x - x1.x, x2.y - x1.y);
    let v2 = Vector2::new(x4.x - x3.x, x4.y - x3.y);

    let m1 = v1.y / v1.x;

    println!("{}", m1);

    let h = x1.y - x1.x * m1;
    // println!("{}", h);
    // println!("{}", x2.y - x2.x * m1);

    let tile_x = 0.0;
    let tile_y = 0.0;

    let side_1 = (tile_x + 1.0) * m1 + h;
    let side_2 = (tile_x + 1.0 - h) / m1;
    let side_3 = (tile_x) * m1 + h;
    let side_4 = (tile_x - h) / m1;

    println!(
        "side_1: {}, side_2: {}, side_3: {}, side_4: {}",
        side_1, side_2, side_3, side_4
    );

    if tile_y <= side_1 && side_1 <= tile_y + 1.0 {
        println!("side1")
    };
    if tile_x <= side_2 && side_2 <= tile_x + 1.0 {
        println!("side2")
    };
    if tile_y <= side_3 && side_3 <= tile_y + 1.0 {
        println!("side3")
    };
    if tile_x <= side_4 && side_4 <= tile_x + 1.0 {
        println!("side4")
    };

    // let diff: Vector2<f32> = x2 - x1;

    // diff.norm()
    // diff.normalize();

    // println!("{:?}", (2.0 as f32).sqrt() / diff.norm() * diff);
    // println!("{:?}", 0.5 * diff);
}
