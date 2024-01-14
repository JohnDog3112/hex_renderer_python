use hex_renderer::{options::{GridPatternOptions, Intersections, Lines}, pattern_utils::Angle};
use pyo3::{pyclass, pymethods, PyResult, Py, PyRef, PyRefMut, exceptions::PyValueError, types::PyModule, Python};

use crate::{initialize_subclass, angles_to_string};

use super::{lines::PyLines, intersections::PyIntersections};

/*pub enum GridPatternOptions {
    Uniform{
        intersections: Intersections,
        lines: Lines
    },
    Changing {
        variations: Vec<(Intersections, Lines)>,
        intros: Vec<String>,
        retros: Vec<String>,
    },
}*/

pub fn initialize_module(py: Python, m: &PyModule) -> PyResult<()> {
    let grid_pattern_options = PyModule::new(py, "GridPatternOptions")?;
    grid_pattern_options.add_class::<GridPatternOptionsUniform>()?;
    grid_pattern_options.add_class::<GridPatternOptionsUniform>()?;
    m.add_submodule(grid_pattern_options)?;

    Ok(())
}


#[derive(Debug, Clone)]
#[pyclass(name = "GridPatternOptions", subclass)]
pub struct PyGridPatternOptions(pub GridPatternOptions);

#[pymethods]
impl PyGridPatternOptions {
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}



impl PyGridPatternOptions {
    pub fn new_py(pattern_options: GridPatternOptions) -> PyResult<Py<Self>> {
        match pattern_options {
            GridPatternOptions::Uniform(intersections, lines) => initialize_subclass(
                GridPatternOptionsUniform::new(PyIntersections(intersections), PyLines(lines)),
            ),
            GridPatternOptions::Changing {
                variations,
                intros,
                retros,
            } => initialize_subclass(
                GridPatternOptionsChanging::new(
                    variations
                        .into_iter()
                        .map(|(a, b)| (PyIntersections(a), PyLines(b)))
                        .collect(),
                    intros.iter().map(angles_to_string).collect(),
                    retros.iter().map(angles_to_string).collect(),
                )
                .expect("This shouldn't be possible"),
            ),
        }
    }
}
#[derive(Debug, Clone)]
#[pyclass(extends=PyGridPatternOptions, name="Uniform")]
struct GridPatternOptionsUniform;

#[pymethods]
impl GridPatternOptionsUniform {
    #[new]
    fn new(intersections: PyIntersections, lines: PyLines) -> (Self, PyGridPatternOptions) {
        (
            Self,
            PyGridPatternOptions(GridPatternOptions::Uniform(intersections.0, lines.0)),
        )
    }

    #[getter]
    fn get_intersections(self_: PyRef<'_, Self>) -> PyResult<Py<PyIntersections>> {
        let super_ = self_.as_ref();

        if let GridPatternOptions::Uniform(intersections, _) = &super_.0 {
            PyIntersections::new_py(*intersections)
        } else {
            unreachable!()
        }
    }

    #[setter]
    fn set_intersections(mut self_: PyRefMut<'_, Self>, intersections_inp: PyIntersections) {
        let super_ = self_.as_mut();

        if let GridPatternOptions::Uniform(intersections, _) = &mut super_.0 {
            *intersections = intersections_inp.0;
        } else {
            unreachable!()
        }
    }

    #[getter]
    fn get_lines(self_: PyRef<'_, Self>) -> PyResult<Py<PyLines>> {
        let super_ = self_.as_ref();

        if let GridPatternOptions::Uniform(_, lines) = &super_.0 {
            PyLines::new_py(lines.clone())
        } else {
            unreachable!()
        }
    }

    #[setter]
    fn set_lines(mut self_: PyRefMut<'_, Self>, lines_inp: PyLines) {
        let super_ = self_.as_mut();

        if let GridPatternOptions::Uniform(_, lines) = &mut super_.0 {
            *lines = lines_inp.0;
        } else {
            unreachable!()
        }
    }
}

#[derive(Debug, Clone)]
#[pyclass(extends=PyGridPatternOptions, name="Changing")]
struct GridPatternOptionsChanging;

/*
Changing {
    variations: Vec<(Intersections, Lines)>,
    intros: Vec<Vec<Angle>>,
    retros: Vec<Vec<Angle>>,
},
*/
#[pymethods]
impl GridPatternOptionsChanging {
    #[new]
    fn new(
        variations: Vec<(PyIntersections, PyLines)>,
        intros: Vec<String>,
        retros: Vec<String>,
    ) -> PyResult<(Self, PyGridPatternOptions)> {
        let variations: Vec<(Intersections, Lines)> =
            variations.into_iter().map(|(a, b)| (a.0, b.0)).collect();

        let intros: Vec<Vec<Angle>> = intros
            .into_iter()
            .map(|str| {
                str.chars()
                    .map(Angle::try_from)
                    .collect::<Result<Vec<Angle>, _>>()
            })
            .collect::<Result<Vec<Vec<Angle>>, _>>()
            .map_err(|err| {
                PyValueError::new_err(format!("Invalid character in intro list! ({})", err.0))
            })?;

        let retros: Vec<Vec<Angle>> = retros
            .into_iter()
            .map(|str| {
                str.chars()
                    .map(Angle::try_from)
                    .collect::<Result<Vec<Angle>, _>>()
            })
            .collect::<Result<Vec<Vec<Angle>>, _>>()
            .map_err(|err| {
                PyValueError::new_err(format!("Invalid character in retro list! ({})", err.0))
            })?;

        Ok((
            Self,
            PyGridPatternOptions(GridPatternOptions::Changing {
                variations,
                intros,
                retros,
            }),
        ))
    }
}