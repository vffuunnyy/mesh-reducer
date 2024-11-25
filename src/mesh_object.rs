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

#[pymethods]
impl MeshObject {
    #[new]
    fn new(name: String, points: Vec<Point>) -> Self {
        MeshObject { name, points }
    }
}
