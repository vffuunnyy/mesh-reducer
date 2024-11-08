use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::{BufReader, Result as IoResult};
use std::path::PathBuf;
use stl_io::read_stl;

#[derive(Clone, Debug)]
struct PointWithNormal {
    point: [f64; 3],
    normal: [f64; 3],
}

fn read_stl_points(file_path: PathBuf) -> IoResult<Vec<PointWithNormal>> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mesh = read_stl(&mut reader)?;
    let vertices = mesh.vertices.clone();

    let points_with_normals = mesh
        .faces
        .into_iter()
        .flat_map(move |face| {
            let normal = face.normal;
            let vertices_clone = vertices.clone();
            face.vertices
                .iter()
                .map(move |&idx| {
                    let vertex = vertices_clone[idx];
                    PointWithNormal {
                        point: [vertex[0] as f64, vertex[1] as f64, vertex[2] as f64],
                        normal: [normal[0] as f64, normal[1] as f64, normal[2] as f64],
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();
    Ok(points_with_normals)
}

fn fast_grid_sampling(
    points_with_normals: Vec<PointWithNormal>,
    clusters: usize,
) -> Vec<PointWithNormal> {
    let mut rng = thread_rng();

    let min_x = points_with_normals
        .iter()
        .map(|p| p.point[0])
        .fold(f64::INFINITY, f64::min);
    let max_x = points_with_normals
        .iter()
        .map(|p| p.point[0])
        .fold(f64::NEG_INFINITY, f64::max);
    let min_y = points_with_normals
        .iter()
        .map(|p| p.point[1])
        .fold(f64::INFINITY, f64::min);
    let max_y = points_with_normals
        .iter()
        .map(|p| p.point[1])
        .fold(f64::NEG_INFINITY, f64::max);
    let min_z = points_with_normals
        .iter()
        .map(|p| p.point[2])
        .fold(f64::INFINITY, f64::min);
    let max_z = points_with_normals
        .iter()
        .map(|p| p.point[2])
        .fold(f64::NEG_INFINITY, f64::max);

    let grid_size = ((clusters as f64).powf(1.0 / 3.0)).ceil() as usize;
    let mut grid: Vec<Vec<PointWithNormal>> = vec![Vec::new(); grid_size * grid_size * grid_size];

    let get_grid_index = |x: f64, y: f64, z: f64| -> usize {
        let gx = ((x - min_x) / (max_x - min_x + f64::EPSILON) * grid_size as f64).floor() as usize;
        let gy = ((y - min_y) / (max_y - min_y + f64::EPSILON) * grid_size as f64).floor() as usize;
        let gz = ((z - min_z) / (max_z - min_z + f64::EPSILON) * grid_size as f64).floor() as usize;
        gx.min(grid_size - 1)
            + gy.min(grid_size - 1) * grid_size
            + gz.min(grid_size - 1) * grid_size * grid_size
    };

    for point_with_normal in points_with_normals.clone() {
        let index = get_grid_index(
            point_with_normal.point[0],
            point_with_normal.point[1],
            point_with_normal.point[2],
        );
        grid[index].push(point_with_normal);
    }

    let mut selected_points = Vec::new();
    for cell in grid.into_iter() {
        if !cell.is_empty() {
            let point_with_normal = cell.choose(&mut rng).unwrap();
            selected_points.push(point_with_normal.clone());
        }
        if selected_points.len() >= clusters {
            break;
        }
    }

    while selected_points.len() < clusters {
        if let Some(point_with_normal) = points_with_normals.choose(&mut rng) {
            selected_points.push(point_with_normal.clone());
        }
    }

    if selected_points.len() > clusters {
        selected_points.shuffle(&mut rng);
        selected_points.truncate(clusters);
    }

    selected_points
}

#[pyfunction(signature = (file_path, clusters))]
fn reduce_stl_points(
    file_path: PathBuf,
    clusters: usize,
) -> PyResult<(Vec<[f64; 3]>, Vec<[f64; 3]>)> {
    let points_with_normals = read_stl_points(file_path).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Ошибка чтения STL-файла: {}", e))
    })?;

    let resampled_points_with_normals = fast_grid_sampling(points_with_normals, clusters);

    let points: Vec<[f64; 3]> = resampled_points_with_normals
        .iter()
        .map(|p| p.point)
        .collect();
    let normals: Vec<[f64; 3]> = resampled_points_with_normals
        .iter()
        .map(|p| p.normal)
        .collect();
    Ok((points, normals))
}

#[pymodule]
fn stl_reducer(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(reduce_stl_points, m)?)?;

    Ok(())
}
