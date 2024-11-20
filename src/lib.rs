mod loader;
mod loaders;
mod mesh_object;
mod progess;
mod sampling;

use loader::MeshLoader;
use loaders::{obj::ObjLoader, ply::PlyLoader, stl::StlLoader};
use sampling::fast_grid_sampling;

use indicatif::ParallelProgressIterator;

use mesh_object::MeshObject;
use progess::create_progess;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rayon::prelude::*;
use std::path::PathBuf;

fn reduce_points_with_loader<L: MeshLoader>(
    file_path: &PathBuf,
    clusters: usize,
) -> Result<MeshObject, String> {
    L::load_points(file_path)
        .map_err(|e| format!("Failed to load {:?}: {:?}", file_path, e))
        .map(|points| MeshObject {
            name: file_path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("")
                .to_string(),
            points: fast_grid_sampling(points, clusters),
        })
}

pub fn reduce_points(file_path: &PathBuf, clusters: usize) -> Result<MeshObject, String> {
    let extension = file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    match extension.as_str() {
        "stl" => reduce_points_with_loader::<StlLoader>(file_path, clusters),
        "obj" => reduce_points_with_loader::<ObjLoader>(file_path, clusters),
        "ply" => reduce_points_with_loader::<PlyLoader>(file_path, clusters),
        _ => Err(format!("Unsupported file format for file {:?}", file_path)),
    }
}

#[pyfunction(signature = (file_path, clusters))]
fn load_mesh(file_path: PathBuf, clusters: usize) -> PyResult<MeshObject> {
    match reduce_points(&file_path, clusters) {
        Ok(mesh) => Ok(mesh),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(e)),
    }
}

#[pyfunction(signature = (file_paths, clusters))]
fn load_meshes(file_paths: Vec<PathBuf>, clusters: usize) -> PyResult<Vec<MeshObject>> {
    let pb = create_progess(file_paths.len() as u64);

    let points: Vec<MeshObject> = file_paths
        .par_iter()
        .progress_with(pb.clone())
        .filter_map(|file_path| match reduce_points(&file_path, clusters) {
            Ok(mesh) => Some(mesh),
            Err(e) => {
                eprintln!("Warning: {}", e);
                None
            }
        })
        .collect();

    pb.finish_with_message("Processing complete");

    Ok(points)
}

#[pyfunction(signature = (file_paths, clusters_range))]
fn load_meshes_range_points(
    file_paths: Vec<PathBuf>,
    mut clusters_range: Vec<usize>,
) -> PyResult<Vec<MeshObject>> {
    if clusters_range.is_empty() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "clusters_range cannot be empty",
        ));
    }

    clusters_range.sort_unstable_by(|a, b| b.cmp(a));

    let total_iterations = file_paths.len() * clusters_range.len();
    let pb = create_progess(total_iterations as u64);

    use std::sync::Mutex;
    let pb = Mutex::new(pb);

    let points: Vec<MeshObject> = file_paths
        .par_iter()
        .filter_map(|file_path| {
            let first_cluster = match clusters_range.first() {
                Some(&c) => c,
                None => {
                    eprintln!("Empty clusters range");
                    return None;
                }
            };

            let first_result = match reduce_points(file_path, first_cluster) {
                Ok(mesh) => mesh,
                Err(e) => {
                    eprintln!("Warning: {}", e);
                    return None;
                }
            };

            let mesh_objects: Vec<MeshObject> = clusters_range
                .par_iter()
                .filter_map(|&cluster| {
                    {
                        pb.lock().unwrap().inc(1);
                    }

                    if cluster == first_cluster {
                        Some(first_result.clone())
                    } else {
                        Some(MeshObject {
                            name: first_result.name.clone(),
                            points: fast_grid_sampling(first_result.points.clone(), cluster),
                        })
                    }
                })
                .collect();

            Some(mesh_objects)
        })
        .flatten()
        .collect();

    {
        pb.lock()
            .unwrap()
            .finish_with_message("Processing complete");
    }

    Ok(points)
}

#[pymodule]
fn mesh_reducer(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load_mesh, m)?)?;
    m.add_function(wrap_pyfunction!(load_meshes, m)?)?;
    m.add_function(wrap_pyfunction!(load_meshes_range_points, m)?)?;
    Ok(())
}
