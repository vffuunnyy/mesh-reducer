use std::path::PathBuf;

use modelz::ModelError;

pub type Point = [f32; 3];

pub trait MeshLoader {
    fn load_points(file_path: &PathBuf) -> Result<Vec<Point>, ModelError>;
}
