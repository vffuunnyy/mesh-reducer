use crate::loader::{MeshLoader, Point};
use modelz::{Model3D, ModelError, ModelFormat};
use std::path::PathBuf;

pub struct ObjLoader;

impl MeshLoader for ObjLoader {
    fn load_points(file_path: &PathBuf) -> Result<Vec<Point>, ModelError> {
        let model = Model3D::from_format(file_path, &ModelFormat::OBJ);

        if let Err(e) = model {
            return Err(e);
        }

        Ok(model
            .unwrap()
            .meshes
            .iter()
            .flat_map(|m| m.vertices.iter())
            .map(|v| v.position)
            .collect())
    }
}
