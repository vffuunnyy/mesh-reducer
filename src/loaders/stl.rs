use crate::loader::{MeshLoader, Point};
use modelz::{Model3D, ModelFormat};
use std::io::Result as IoResult;
use std::path::PathBuf;

pub struct StlLoader;

impl MeshLoader for StlLoader {
    fn load_points(file_path: PathBuf) -> IoResult<Vec<Point>> {
        let model = Model3D::from_format(file_path, &ModelFormat::STL).expect("Failed to load");

        let points: Vec<Point> = model
            .meshes
            .iter()
            .flat_map(|m| m.vertices.iter())
            .map(|v| v.position)
            .collect();

        Ok(points)
    }
}
