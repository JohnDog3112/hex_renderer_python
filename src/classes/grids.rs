use std::fs;

use hex_renderer::{grids::{GridDraw, GridDrawError, SquareGrid, HexGrid}, PatternVariant};
use pyo3::{pyclass, PyErr, exceptions::PyValueError, FromPyObject, PyRef, pymethods, PyResult, types::PyModule};

use super::{grid_options::PyGridOptions, pattern_variant::PyPatternVariant};


pub fn initialize_classes(m: &PyModule) -> PyResult<()> {
    m.add_class::<PyHexGrid>()?;
    m.add_class::<PySquareGrid>()?;

    Ok(())
}

#[pyclass(name = "Grid", subclass)]
struct PyGrid(Box<dyn GridDraw + Send>);

fn map_draw_error(err: GridDrawError) -> PyErr {
    match err {
        GridDrawError::ImproperScale(f32) => {
            PyValueError::new_err(format!("{f32} isn't a valid scale!"))
        }
        GridDrawError::EncodeError => {
            PyValueError::new_err("Something went wrong and the grid couldn't be drawn.")
        }
    }
}

#[derive(FromPyObject)]
enum ScaleOption<'a> {
    Padding(f32),
    Options(PyRef<'a, PyGridOptions>),
}
#[pymethods]
impl PyGrid {
    fn draw_png(
        &self,
        scale: f32,
        options: PyRef<'_, PyGridOptions>,
        padding: Option<f32>,
    ) -> PyResult<Vec<u8>> {
        let padding = match padding {
            Some(pad) => pad,
            None => options.0.get_max_radius(),
        };

        self.0
            .draw_grid_with_padding(scale, &options.0, padding)
            .map_err(map_draw_error)?
            .encode_png()
            .map_err(|_| PyValueError::new_err("Failed to encode into png!"))
    }

    fn draw_to_file(
        &self,
        file_name: &str,
        scale: f32,
        options: PyRef<'_, PyGridOptions>,
        padding: Option<f32>,
    ) -> PyResult<()> {
        fs::write(file_name, self.draw_png(scale, options, padding)?)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    fn get_bound_scale(&self, bound: (f32, f32), options: ScaleOption) -> f32 {
        let size = self.0.get_unpadded_size();

        let padding = match options {
            ScaleOption::Padding(pad) => pad,
            ScaleOption::Options(grid) => grid.0.get_max_radius(),
        };

        let size = (padding * 2.0 + size.0, padding * 2.0 + size.1);

        (bound.0 / size.0).min(bound.1 / size.1).max(1.0)
    }
}

#[pyclass(name="HexGrid", extends=PyGrid)]
struct PyHexGrid;

#[pymethods]
impl PyHexGrid {
    #[new]
    fn new(patterns: Vec<PyPatternVariant>, max_width: usize) -> PyResult<(Self, PyGrid)> {
        let patterns = patterns.into_iter().map(PatternVariant::from).collect();

        let grid = HexGrid::new(patterns, max_width)
            .map_err(|_| PyValueError::new_err("Failed to create grid!"))?;

        Ok((Self, PyGrid(Box::new(grid))))
    }
}

#[pyclass(name="SquareGrid", extends=PyGrid)]
struct PySquareGrid;

#[pymethods]
impl PySquareGrid {
    #[new]
    fn new(
        patterns: Vec<PyPatternVariant>,
        max_width: usize,
        max_scale: f32,
        x_pad: f32,
        y_pad: f32,
    ) -> PyResult<(Self, PyGrid)> {
        let patterns = patterns.into_iter().map(PatternVariant::from).collect();

        let grid = SquareGrid::new(patterns, max_width, max_scale, x_pad, y_pad)
            .map_err(|_| PyValueError::new_err("Failed to create grid!"))?;

        Ok((Self, PyGrid(Box::new(grid))))
    }
}