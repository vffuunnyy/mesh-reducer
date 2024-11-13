mod loader;
mod loaders;

use loader::MeshLoader;
use loader::Point;
use loaders::{obj::ObjLoader, ply::PlyLoader, stl::StlLoader};

use dashmap::DashMap;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;
use std::io::Result as IoResult;
use std::path::PathBuf;

fn reduce_points_with_loader<L: MeshLoader>(
    file_path: PathBuf,
    clusters: usize,
) -> IoResult<Vec<Point>> {
    let points = L::load_points(file_path)?;
    Ok(fast_grid_sampling(points, clusters))
}

pub fn reduce_points(file_path: PathBuf, clusters: usize) -> IoResult<Vec<Point>> {
    let extension = file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    match extension.as_str() {
        "stl" => reduce_points_with_loader::<StlLoader>(file_path, clusters),
        "obj" => reduce_points_with_loader::<ObjLoader>(file_path, clusters),
        "ply" => reduce_points_with_loader::<PlyLoader>(file_path, clusters),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Unsupported file format",
        )),
    }
}

fn fast_grid_sampling(points: Vec<Point>, clusters: usize) -> Vec<Point> {
    let (min_x, max_x, min_y, max_y, min_z, max_z) = points
        .par_iter()
        .map(|p| {
            let x = p[0];
            let y = p[1];
            let z = p[2];
            (x, x, y, y, z, z)
        })
        .reduce(
            || {
                (
                    f32::INFINITY,
                    f32::NEG_INFINITY,
                    f32::INFINITY,
                    f32::NEG_INFINITY,
                    f32::INFINITY,
                    f32::NEG_INFINITY,
                )
            },
            |a, b| {
                (
                    a.0.min(b.0),
                    a.1.max(b.1),
                    a.2.min(b.2),
                    a.3.max(b.3),
                    a.4.min(b.4),
                    a.5.max(b.5),
                )
            },
        );

    let grid_size = ((clusters as f32).powf(1.0 / 3.0)).ceil() as usize;

    let grid = DashMap::with_capacity(clusters);

    let denom_x = max_x - min_x + f32::EPSILON;
    let denom_y = max_y - min_y + f32::EPSILON;
    let denom_z = max_z - min_z + f32::EPSILON;

    let get_grid_index = |x: f32, y: f32, z: f32| -> usize {
        let gx = ((x - min_x) / denom_x * grid_size as f32).floor() as usize;
        let gy = ((y - min_y) / denom_y * grid_size as f32).floor() as usize;
        let gz = ((z - min_z) / denom_z * grid_size as f32).floor() as usize;
        let gx = gx.min(grid_size - 1);
        let gy = gy.min(grid_size - 1);
        let gz = gz.min(grid_size - 1);
        gx + gy * grid_size + gz * grid_size * grid_size
    };

    points.par_iter().for_each(|&point| {
        let index = get_grid_index(point[0], point[1], point[2]);
        grid.entry(index).or_insert_with(Vec::new).push(point);
    });

    let mut selected_points: Vec<Point> = grid
        .into_iter()
        .par_bridge()
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

#[pyfunction(signature = (file_path, clusters))]
fn load_mesh(file_path: PathBuf, clusters: usize) -> PyResult<Vec<Point>> {
    reduce_points(file_path, clusters).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Error reducing mesh points: {}", e))
    })
}

#[pyfunction(signature = (file_paths, clusters))]
fn load_meshes(file_paths: Vec<PathBuf>, clusters: usize) -> PyResult<Vec<Vec<Point>>> {
    let results: Vec<IoResult<Vec<Point>>> = file_paths
        .into_par_iter()
        .map(|file_path| reduce_points(file_path, clusters))
        .collect();

    let mut points = Vec::new();
    for result in results {
        match result {
            Ok(p) => points.push(p),
            Err(e) => {
                return Err(PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                    "Error reducing mesh points: {}",
                    e
                )))
            }
        }
    }

    Ok(points)
}

#[pymodule]
fn mesh_reducer(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load_mesh, m)?)?;
    m.add_function(wrap_pyfunction!(load_meshes, m)?)?;
    Ok(())
}
