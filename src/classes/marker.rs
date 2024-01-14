use hex_renderer::options::Marker;
use pyo3::{pyclass, pymethods};

use crate::PyColor;

#[pyclass(name = "Marker")]
#[derive(Debug, Clone)]
pub struct PyMarker(pub Marker);

#[pymethods]
impl PyMarker {
    #[new]
    fn new(color: PyColor, radius: f32) -> Self {
        Self(Marker {
            color: color.0,
            radius,
        })
    }
    #[getter]
    fn get_color(&self) -> PyColor {
        PyColor(self.0.color)
    }
    #[setter]
    fn set_color(&mut self, color: PyColor) {
        self.0.color = color.0;
    }
    #[getter]
    fn get_radius(&self) -> f32 {
        self.0.radius
    }
    #[setter]
    fn set_radius(&mut self, radius: f32) {
        self.0.radius = radius;
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}