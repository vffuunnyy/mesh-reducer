use dashmap::DashMap;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Result as IoResult};
use std::path::PathBuf;
use stl_io::read_stl;

#[derive(Clone, Debug)]
struct PointWithNormal {
    point: [f32; 3],
    normal: [f32; 3],
}

fn read_stl_points(file_path: PathBuf) -> IoResult<Vec<PointWithNormal>> {
    let t = std::time::Instant::now();
    let file = File::open(&file_path)?;
    let mut reader = BufReader::new(file);
    let mesh = read_stl(&mut reader)?;
    let vertices = &mesh.vertices;

    type NormalAccum = HashMap<usize, [f32; 3]>;

    let normal_map = mesh
        .faces
        .par_iter()
        .map(|face| {
            let normal = [
                face.normal[0] as f32,
                face.normal[1] as f32,
                face.normal[2] as f32,
            ];
            let mut map = NormalAccum::new();
            for &idx in &face.vertices {
                map.entry(idx)
                    .and_modify(|acc_normal| {
                        acc_normal[0] += normal[0];
                        acc_normal[1] += normal[1];
                        acc_normal[2] += normal[2];
                    })
                    .or_insert(normal);
            }
            map
        })
        .reduce(
            || NormalAccum::new(),
            |mut acc, map| {
                for (idx, normal) in map {
                    acc.entry(idx)
                        .and_modify(|acc_normal| {
                            acc_normal[0] += normal[0];
                            acc_normal[1] += normal[1];
                            acc_normal[2] += normal[2];
                        })
                        .or_insert(normal);
                }
                acc
            },
        );

    let points_with_normals: Vec<PointWithNormal> = normal_map
        .into_par_iter()
        .map(|(idx, summed_normal)| {
            let magnitude =
                (summed_normal[0].powi(2) + summed_normal[1].powi(2) + summed_normal[2].powi(2))
                    .sqrt();
            let normalized_normal = if magnitude != 0.0 {
                [
                    summed_normal[0] / magnitude,
                    summed_normal[1] / magnitude,
                    summed_normal[2] / magnitude,
                ]
            } else {
                [0.0, 0.0, 0.0]
            };

            let vertex = vertices[idx];
            PointWithNormal {
                point: [vertex[0] as f32, vertex[1] as f32, vertex[2] as f32],
                normal: normalized_normal,
            }
        })
        .collect();

    println!("Reading STL took {:?}", t.elapsed());
    Ok(points_with_normals)
}

fn fast_grid_sampling(
    points_with_normals: Vec<PointWithNormal>,
    clusters: usize,
) -> Vec<PointWithNormal> {
    let t = std::time::Instant::now();
    let min_x = points_with_normals
        .par_iter()
        .map(|p| p.point[0])
        .reduce_with(f32::min)
        .unwrap_or(f32::INFINITY);
    let max_x = points_with_normals
        .par_iter()
        .map(|p| p.point[0])
        .reduce(|| f32::NEG_INFINITY, f32::max);

    let min_y = points_with_normals
        .par_iter()
        .map(|p| p.point[1])
        .reduce(|| f32::INFINITY, f32::min);
    let max_y = points_with_normals
        .par_iter()
        .map(|p| p.point[1])
        .reduce(|| f32::NEG_INFINITY, f32::max);

    let min_z = points_with_normals
        .par_iter()
        .map(|p| p.point[2])
        .reduce(|| f32::INFINITY, f32::min);
    let max_z = points_with_normals
        .par_iter()
        .map(|p| p.point[2])
        .reduce(|| f32::NEG_INFINITY, f32::max);

    let grid_size = ((clusters as f32).powf(1.0 / 3.0)).ceil() as usize;

    let grid = DashMap::new();

    let get_grid_index = |x: f32, y: f32, z: f32| -> usize {
        let gx = ((x - min_x) / (max_x - min_x + f32::EPSILON) * grid_size as f32).floor() as usize;
        let gy = ((y - min_y) / (max_y - min_y + f32::EPSILON) * grid_size as f32).floor() as usize;
        let gz = ((z - min_z) / (max_z - min_z + f32::EPSILON) * grid_size as f32).floor() as usize;
        gx.min(grid_size - 1)
            + gy.min(grid_size - 1) * grid_size
            + gz.min(grid_size - 1) * grid_size * grid_size
    };

    points_with_normals
        .par_iter()
        .for_each(|point_with_normal| {
            let index = get_grid_index(
                point_with_normal.point[0],
                point_with_normal.point[1],
                point_with_normal.point[2],
            );
            grid.entry(index)
                .or_insert_with(Vec::new)
                .push(point_with_normal.clone());
        });

    let mut selected_points: Vec<PointWithNormal> = grid
        .iter()
        .filter_map(|entry| {
            let mut rng = thread_rng();
            let cell = entry.value();
            if !cell.is_empty() {
                Some(cell.choose(&mut rng).unwrap().clone())
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
            if let Some(point_with_normal) = points_with_normals.choose(&mut rng) {
                selected_points.push(point_with_normal.clone());
            }
        }
    }

    println!("Fast grid sampling took {:?}", t.elapsed());

    selected_points
}

#[pyfunction(signature = (file_path, clusters))]
fn reduce_stl_points(
    file_path: PathBuf,
    clusters: usize,
) -> PyResult<(Vec<[f32; 3]>, Vec<[f32; 3]>)> {
    let points_with_normals = read_stl_points(file_path).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Ошибка чтения STL-файла: {}", e))
    })?;

    let resampled_points_with_normals = fast_grid_sampling(points_with_normals, clusters);

    let (points, normals): (Vec<_>, Vec<_>) = resampled_points_with_normals
        .par_iter()
        .map(|p| (p.point, p.normal))
        .unzip();

    Ok((points, normals))
}

#[pymodule]
fn stl_reducer(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(reduce_stl_points, m)?)?;

    Ok(())
}
