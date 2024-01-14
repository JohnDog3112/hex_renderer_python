use hex_renderer::options::Color;
use pyo3::{pyclass, pymethods};


#[pyclass(name = "Color")]
#[derive(Debug, Clone, Copy)]
pub struct PyColor(pub Color);

#[pymethods]
impl PyColor {
    #[new]
    fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(Color(r, g, b, a))
    }

    #[getter]
    fn get_r(&self) -> u8 {
        self.0 .0
    }
    #[setter]
    fn set_r(&mut self, r: u8) {
        self.0 .0 = r;
    }
    #[getter]
    fn get_g(&self) -> u8 {
        self.0 .1
    }
    #[setter]
    fn set_g(&mut self, g: u8) {
        self.0 .1 = g;
    }
    #[getter]
    fn get_b(&self) -> u8 {
        self.0 .2
    }
    #[setter]
    fn set_b(&mut self, b: u8) {
        self.0 .2 = b;
    }
    #[getter]
    fn get_a(&self) -> u8 {
        self.0 .3
    }
    #[setter]
    fn set_a(&mut self, a: u8) {
        self.0 .3 = a;
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}