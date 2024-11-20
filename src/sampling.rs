use crate::loader::Point;

use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;
use std::collections::HashMap;

pub fn fast_grid_sampling(points: Vec<Point>, clusters: usize) -> Vec<Point> {
    let (min_x, max_x, min_y, max_y, min_z, max_z) = points.iter().fold(
        (
            f32::INFINITY,
            f32::NEG_INFINITY,
            f32::INFINITY,
            f32::NEG_INFINITY,
            f32::INFINITY,
            f32::NEG_INFINITY,
        ),
        |(min_x, max_x, min_y, max_y, min_z, max_z), p| {
            (
                min_x.min(p[0]),
                max_x.max(p[0]),
                min_y.min(p[1]),
                max_y.max(p[1]),
                min_z.min(p[2]),
                max_z.max(p[2]),
            )
        },
    );

    let grid_size = ((clusters as f32).powf(1.0 / 3.0)).ceil() as usize;

    let denom_x = max_x - min_x + f32::EPSILON;
    let denom_y = max_y - min_y + f32::EPSILON;
    let denom_z = max_z - min_z + f32::EPSILON;

    let inv_denom_x = 1.0 / denom_x;
    let inv_denom_y = 1.0 / denom_y;
    let inv_denom_z = 1.0 / denom_z;

    let grid_size_f32 = grid_size as f32;

    let scale_x = grid_size_f32 * inv_denom_x;
    let scale_y = grid_size_f32 * inv_denom_y;
    let scale_z = grid_size_f32 * inv_denom_z;

    let get_grid_index = |x: f32, y: f32, z: f32| -> usize {
        let gx = ((x - min_x) * scale_x).floor() as usize;
        let gy = ((y - min_y) * scale_y).floor() as usize;
        let gz = ((z - min_z) * scale_z).floor() as usize;
        let gx = gx.min(grid_size - 1);
        let gy = gy.min(grid_size - 1);
        let gz = gz.min(grid_size - 1);
        gx + gy * grid_size + gz * grid_size * grid_size
    };

    let grid = points
        .par_iter()
        .fold(
            || HashMap::new(),
            |mut acc, &point| {
                let index = get_grid_index(point[0], point[1], point[2]);
                acc.entry(index).or_insert_with(Vec::new).push(point);
                acc
            },
        )
        .reduce(
            || HashMap::new(),
            |mut acc, map| {
                for (k, v) in map {
                    acc.entry(k).or_insert_with(Vec::new).extend(v);
                }
                acc
            },
        );

    let mut selected_points: Vec<Point> = grid
        .into_par_iter()
        .filter_map(|(_key, cell)| {
            let mut rng = thread_rng();
            if !cell.is_empty() {
                Some(*cell.choose(&mut rng).unwrap())
            } else {
                None
            }
        })
        .collect();

    let mut rng = thread_rng();
    if selected_points.len() > clusters {
        selected_points.shuffle(&mut rng);
        selected_points.truncate(clusters);
    } else {
        while selected_points.len() < clusters {
            if let Some(&point) = points.choose(&mut rng) {
                selected_points.push(point);
            }
        }
    }

    selected_points
}
