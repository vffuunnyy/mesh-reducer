use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::{BufReader, Result as IoResult};
use std::path::PathBuf;
use stl_io::read_stl;

fn read_stl_points(file_path: PathBuf) -> IoResult<Vec<[f64; 3]>> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mesh = read_stl(&mut reader)?;
    let points = mesh
        .faces
        .iter()
        .flat_map(|face| {
            face.vertices.iter().map(|&idx| {
                let vertex = mesh.vertices[idx];
                [vertex[0] as f64, vertex[1] as f64, vertex[2] as f64]
            })
        })
        .collect();
    Ok(points)
}

fn fast_grid_sampling(points: Vec<[f64; 3]>, clusters: usize) -> Vec<[f64; 3]> {
    let mut rng = thread_rng();

    let min_x = points.iter().map(|p| p[0]).fold(f64::INFINITY, f64::min);
    let max_x = points
        .iter()
        .map(|p| p[0])
        .fold(f64::NEG_INFINITY, f64::max);
    let min_y = points.iter().map(|p| p[1]).fold(f64::INFINITY, f64::min);
    let max_y = points
        .iter()
        .map(|p| p[1])
        .fold(f64::NEG_INFINITY, f64::max);
    let min_z = points.iter().map(|p| p[2]).fold(f64::INFINITY, f64::min);
    let max_z = points
        .iter()
        .map(|p| p[2])
        .fold(f64::NEG_INFINITY, f64::max);

    let grid_size = ((clusters as f64).powf(1.0 / 3.0)).ceil() as usize;
    let mut grid: Vec<Vec<[f64; 3]>> = vec![Vec::new(); grid_size * grid_size * grid_size];

    let get_grid_index = |x: f64, y: f64, z: f64| -> usize {
        let gx = ((x - min_x) / (max_x - min_x + f64::EPSILON) * grid_size as f64).floor() as usize;
        let gy = ((y - min_y) / (max_y - min_y + f64::EPSILON) * grid_size as f64).floor() as usize;
        let gz = ((z - min_z) / (max_z - min_z + f64::EPSILON) * grid_size as f64).floor() as usize;
        gx.min(grid_size - 1)
            + gy.min(grid_size - 1) * grid_size
            + gz.min(grid_size - 1) * grid_size * grid_size
    };

    for point in points.clone() {
        let index = get_grid_index(point[0], point[1], point[2]);
        grid[index].push(point);
    }

    let mut selected_points = Vec::new();
    for cell in grid.into_iter() {
        if !cell.is_empty() {
            let point = cell.choose(&mut rng).unwrap();
            selected_points.push(*point);
        }
        if selected_points.len() >= clusters {
            break;
        }
    }

    while selected_points.len() < clusters {
        if let Some(point) = points.choose(&mut rng) {
            selected_points.push(*point);
        }
    }

    if selected_points.len() > clusters {
        selected_points.shuffle(&mut rng);
        selected_points.truncate(clusters);
    }

    selected_points
}

#[pyfunction(signature = (file_path, clusters))]
fn reduce_stl_points(file_path: PathBuf, clusters: usize) -> PyResult<Vec<[f64; 3]>> {
    let points = read_stl_points(file_path).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Ошибка чтения STL-файла: {}", e))
    })?;

    Ok(fast_grid_sampling(points, clusters))
}

#[pymodule]
fn stl_reducer(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(reduce_stl_points, m)?)?;

    Ok(())
}
