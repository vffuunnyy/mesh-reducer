use crate::loader::{MeshLoader, Point};
use stl_io::read_stl;
use std::fs::File;
use std::io::BufReader;
use std::io::Result as IoResult;
use std::path::PathBuf;

pub struct StlLoader;

impl MeshLoader for StlLoader {
    fn load_points(file_path: PathBuf) -> IoResult<Vec<Point>> {
        let file = File::open(&file_path)?;
        let mut reader = BufReader::new(file);
        let mesh = read_stl(&mut reader)?;
        let points: Vec<Point> = mesh.vertices.iter().map(|v| [v[0], v[1], v[2]]).collect();
        Ok(points)
    }
}
