/*pub struct GridOptions {
    pub line_thickness: f32,

    pub pattern_options: GridPatternOptions,

    pub center_dot: Point,
}*/

use hex_renderer::options::GridOptions;
use pyo3::{pyclass, pymethods, PyResult, Py};

use super::{grid_pattern_options::PyGridPatternOptions, point::PyPoint};

#[pyclass(name = "GridOptions")]
#[derive(Debug, Clone)]
pub struct PyGridOptions(pub GridOptions);

#[pymethods]
impl PyGridOptions {
    #[new]
    fn new(
        line_thickness: f32,
        pattern_options: PyGridPatternOptions,
        center_dot: PyPoint,
    ) -> Self {
        Self(GridOptions {
            line_thickness,
            pattern_options: pattern_options.0,
            center_dot: center_dot.0,
        })
    }

    #[getter]
    fn get_line_thickness(&self) -> f32 {
        self.0.line_thickness
    }
    #[setter]
    fn set_line_thickness(&mut self, line_thickness: f32) {
        self.0.line_thickness = line_thickness;
    }

    #[getter]
    fn get_pattern_options(&self) -> PyResult<Py<PyGridPatternOptions>> {
        PyGridPatternOptions::new_py(self.0.pattern_options.clone())
    }
    #[setter]
    fn set_pattern_options(&mut self, pattern_options: PyGridPatternOptions) {
        self.0.pattern_options = pattern_options.0;
    }

    #[getter]
    fn get_center_dot(&self) -> PyResult<Py<PyPoint>> {
        PyPoint::new_py(self.0.center_dot)
    }
    #[setter]
    fn set_center_dot(&mut self, center_dot: PyPoint) {
        self.0.center_dot = center_dot.0;
    }
}