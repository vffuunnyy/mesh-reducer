use std::io::Result as IoResult;
use std::path::PathBuf;

pub type Point = [f32; 3];

pub trait MeshLoader {
    fn load_points(file_path: PathBuf) -> IoResult<Vec<Point>>;
}
