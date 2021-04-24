use bevy_rapier2d::na::Vector2;
use rand::prelude::*;

pub fn rectangular_area(
    min_side_length: usize,
    max_side_length: usize,
) -> Vec<(isize, isize, usize)> {
    let mut tiles = Vec::new();
    // tiles.push((1, 1));

    let height = rand::thread_rng().gen_range(min_side_length..max_side_length);
    let width = rand::thread_rng().gen_range(min_side_length..max_side_length);

    for i in 0..height {
        for j in 0..width {
            tiles.push((
                j as isize - width as isize / 2,
                i as isize - height as isize / 2,
                match (i, j) {
                    (i, j) if i == 0 && j == 0 => 32,
                    (i, j) if i == 0 && j == width - 1 => 34,
                    (i, j) if i == height - 1 && j == 0 => 0,
                    (i, j) if i == height - 1 && j == width - 1 => 2,
                    (i, _) if i == 0 => 1,
                    (_, j) if j == 0 => 16,
                    (_, j) if j == width - 1 => 18,
                    (i, _) if i == height - 1 => 33,
                    _ => 17,
                },
            ));
        }
    }

    tiles
}
