use crate::loader::{MeshLoader, Point};
use obj::{load_obj, Obj, Position};
use std::fs::File;
use std::io::BufReader;
use std::io::Result as IoResult;
use std::path::PathBuf;

pub struct ObjLoader;

impl MeshLoader for ObjLoader {
    fn load_points(file_path: PathBuf) -> IoResult<Vec<Point>> {
        let file = File::open(&file_path)?;
        let mut reader = BufReader::new(file);
        let obj: Obj<Position> =
            load_obj(&mut reader).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        let points: Vec<Point> = obj
            .vertices
            .iter()
            .map(|v| [v.position[0], v.position[1], v.position[2]])
            .collect();
        Ok(points)
    }
}
