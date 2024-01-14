use hex_renderer::options::CollisionOption;
use pyo3::{pyclass, pymethods, PyResult, Py, PyRef, PyRefMut, Python, types::PyModule};

use crate::initialize_subclass;

use super::{color::PyColor, overload_options::PyOverloadOptions};

/*pub enum CollisionOption {
    Dashes {
        color: Color
    },
    MatchedDashes,
    ParallelLines,
    OverloadedParallel {
        max_line: usize,
        overload: OverloadOptions,
    },
}*/

pub fn initialize_module(py: Python, m: &PyModule) -> PyResult<()> {
    let collision_option = PyModule::new(py, "CollisionOption")?;
    collision_option.add_class::<CollisionOptionDashes>()?;
    collision_option.add_class::<CollisionOptionMatchedDashes>()?;
    collision_option.add_class::<CollisionOptionOverloadedParallel>()?;
    collision_option.add_class::<CollisionOptionParallelLines>()?;
    m.add_submodule(collision_option)?;

    Ok(())
}

#[derive(Debug, Clone)]
#[pyclass(name = "CollisionOption", subclass)]
pub struct PyCollisionOption(pub CollisionOption);

#[pymethods]
impl PyCollisionOption {
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

impl PyCollisionOption {
    pub fn new_py(collision_option: CollisionOption) -> PyResult<Py<Self>> {
        match collision_option {
            CollisionOption::Dashes(color) => {
                initialize_subclass(CollisionOptionDashes::new(PyColor(color)))
            }
            CollisionOption::MatchedDashes => {
                initialize_subclass(CollisionOptionMatchedDashes::new())
            }
            CollisionOption::ParallelLines => {
                initialize_subclass(CollisionOptionParallelLines::new())
            }
            CollisionOption::OverloadedParallel { max_line, overload } => initialize_subclass(
                CollisionOptionOverloadedParallel::new(max_line, PyOverloadOptions(overload)),
            ),
        }
    }
}

#[derive(Debug, Clone)]
#[pyclass(name="Dashes", extends=PyCollisionOption)]
struct CollisionOptionDashes;

#[pymethods]
impl CollisionOptionDashes {
    #[new]
    fn new(color: PyColor) -> (Self, PyCollisionOption) {
        (Self, PyCollisionOption(CollisionOption::Dashes(color.0)))
    }

    #[getter]
    fn get_color(self_: PyRef<'_, Self>) -> PyColor {
        let super_ = self_.as_ref();

        if let CollisionOption::Dashes(_color) = &super_.0 {
            PyColor(*_color)
        } else {
            unreachable!()
        }
    }

    #[setter]
    fn set_color(mut self_: PyRefMut<'_, Self>, color_inp: PyColor) {
        let super_ = self_.as_mut();

        if let CollisionOption::Dashes(_color) = &mut super_.0 {
            *_color = color_inp.0;
        } else {
            unreachable!()
        }
    }
}
#[derive(Debug, Clone)]
#[pyclass(name="MatchedDashes", extends=PyCollisionOption)]
struct CollisionOptionMatchedDashes;

#[pymethods]
impl CollisionOptionMatchedDashes {
    #[new]
    fn new() -> (Self, PyCollisionOption) {
        (Self, PyCollisionOption(CollisionOption::MatchedDashes {}))
    }
}
#[derive(Debug, Clone)]
#[pyclass(name="ParallelLines", extends=PyCollisionOption)]
struct CollisionOptionParallelLines;

#[pymethods]
impl CollisionOptionParallelLines {
    #[new]
    fn new() -> (Self, PyCollisionOption) {
        (Self, PyCollisionOption(CollisionOption::ParallelLines {}))
    }
}
#[derive(Debug, Clone)]
#[pyclass(name="OverloadedParallel", extends=PyCollisionOption)]
struct CollisionOptionOverloadedParallel;

#[pymethods]
impl CollisionOptionOverloadedParallel {
    #[new]
    fn new(max_line: usize, overload: PyOverloadOptions) -> (Self, PyCollisionOption) {
        (
            Self,
            PyCollisionOption(CollisionOption::OverloadedParallel {
                max_line,

                overload: overload.0,
            }),
        )
    }

    #[getter]
    fn get_max_line(self_: PyRef<'_, Self>) -> usize {
        let super_ = self_.as_ref();

        if let CollisionOption::OverloadedParallel {
            max_line: _max_line,
            overload: _overload,
        } = &super_.0
        {
            *_max_line
        } else {
            unreachable!()
        }
    }

    #[setter]
    fn set_max_line(mut self_: PyRefMut<'_, Self>, max_line_inp: usize) {
        let super_ = self_.as_mut();

        if let CollisionOption::OverloadedParallel {
            max_line: _max_line,
            overload: _overload,
        } = &mut super_.0
        {
            *_max_line = max_line_inp;
        } else {
            unreachable!()
        }
    }
    #[getter]
    fn get_overload(self_: PyRef<'_, Self>) -> PyResult<Py<PyOverloadOptions>> {
        let super_ = self_.as_ref();

        if let CollisionOption::OverloadedParallel {
            max_line: _max_line,
            overload: _overload,
        } = &super_.0
        {
            PyOverloadOptions::new_py(*_overload)
        } else {
            unreachable!()
        }
    }

    #[setter]
    fn set_overload(mut self_: PyRefMut<'_, Self>, overload_inp: PyOverloadOptions) {
        let super_ = self_.as_mut();

        if let CollisionOption::OverloadedParallel {
            max_line: _max_line,
            overload: _overload,
        } = &mut super_.0
        {
            *_overload = overload_inp.0;
        } else {
            unreachable!()
        }
    }
}