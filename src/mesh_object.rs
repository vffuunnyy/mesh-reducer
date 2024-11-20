use pyo3::prelude::*;

use crate::loader::Point;

#[pyclass]
#[derive(Clone)]
pub struct MeshObject {
    #[pyo3(get)]
    pub name: String,

    #[pyo3(get)]
    pub points: Vec<Point>,
}