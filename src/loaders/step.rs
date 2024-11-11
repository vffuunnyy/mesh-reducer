use crate::loader::{MeshLoader, Point};
use std::io::Result as IoResult;
use std::path::PathBuf;

pub struct StepLoader;

impl MeshLoader for StepLoader {
    fn load_points(_file_path: PathBuf) -> IoResult<Vec<Point>> {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "STEP file support not yet implemented",
        ))
    }
}
