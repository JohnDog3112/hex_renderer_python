use hex_renderer::options::OverloadOptions;
use pyo3::{pyclass, pymethods, PyResult, Py, PyRef, PyRefMut, Python, types::PyModule};

use crate::initialize_subclass;

use super::{color::PyColor, marker::PyMarker};

/*pub enum OverloadOptions {
    Dashes{ color: Color},
    LabeledDashes { color: Color, label: Marker },
    MatchedDashes,
}*/

pub fn initialize_module(py: Python, m: &PyModule) -> PyResult<()> {
    let overload_options = PyModule::new(py, "OverloadOptions")?;
    overload_options.add_class::<OverloadOptionsDashes>()?;
    overload_options.add_class::<OverloadOptionsLabeledDashes>()?;
    overload_options.add_class::<OverloadOptionsMatchedDashes>()?;
    m.add_submodule(overload_options)?;

    Ok(())
}

#[pyclass(name = "OverloadOptions", subclass)]
#[derive(Debug, Clone)]
pub struct PyOverloadOptions(pub OverloadOptions);

#[pymethods]
impl PyOverloadOptions {
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

impl PyOverloadOptions {
    pub fn new_py(overload_options: OverloadOptions) -> PyResult<Py<Self>> {
        match overload_options {
            OverloadOptions::Dashes(dashes) => {
                initialize_subclass(OverloadOptionsDashes::new(PyColor(dashes)))
            }
            OverloadOptions::LabeledDashes { color, label } => initialize_subclass(
                OverloadOptionsLabeledDashes::new(PyColor(color), PyMarker(label)),
            ),
            OverloadOptions::MatchedDashes => {
                initialize_subclass(OverloadOptionsMatchedDashes::new())
            }
        }
    }
}

#[derive(Debug, Clone)]
#[pyclass(name="Dashes", extends=PyOverloadOptions)]
struct OverloadOptionsDashes;

#[pymethods]
impl OverloadOptionsDashes {
    #[new]
    fn new(color: PyColor) -> (Self, PyOverloadOptions) {
        (Self, PyOverloadOptions(OverloadOptions::Dashes(color.0)))
    }

    #[getter]
    fn get_color(self_: PyRef<'_, Self>) -> PyColor {
        let super_ = self_.as_ref();

        if let OverloadOptions::Dashes(_color) = &super_.0 {
            PyColor(*_color)
        } else {
            unreachable!()
        }
    }

    #[setter]
    fn set_color(mut self_: PyRefMut<'_, Self>, color_inp: PyColor) {
        let super_ = self_.as_mut();

        if let OverloadOptions::Dashes(_color) = &mut super_.0 {
            *_color = color_inp.0;
        } else {
            unreachable!()
        }
    }
}
#[derive(Debug, Clone)]
#[pyclass(name="LabeledDashes", extends=PyOverloadOptions)]
struct OverloadOptionsLabeledDashes;

#[pymethods]
impl OverloadOptionsLabeledDashes {
    #[new]
    fn new(color: PyColor, label: PyMarker) -> (Self, PyOverloadOptions) {
        (
            Self,
            PyOverloadOptions(OverloadOptions::LabeledDashes {
                color: color.0,

                label: label.0,
            }),
        )
    }

    #[getter]
    fn get_color(self_: PyRef<'_, Self>) -> PyColor {
        let super_ = self_.as_ref();

        if let OverloadOptions::LabeledDashes {
            color: _color,
            label: _label,
        } = &super_.0
        {
            PyColor(*_color)
        } else {
            unreachable!()
        }
    }

    #[setter]
    fn set_color(mut self_: PyRefMut<'_, Self>, color_inp: PyColor) {
        let super_ = self_.as_mut();

        if let OverloadOptions::LabeledDashes {
            color: _color,
            label: _label,
        } = &mut super_.0
        {
            *_color = color_inp.0;
        } else {
            unreachable!()
        }
    }
    #[getter]
    fn get_label(self_: PyRef<'_, Self>) -> PyMarker {
        let super_ = self_.as_ref();

        if let OverloadOptions::LabeledDashes {
            color: _color,
            label: _label,
        } = &super_.0
        {
            PyMarker(*_label)
        } else {
            unreachable!()
        }
    }

    #[setter]
    fn set_label(mut self_: PyRefMut<'_, Self>, label_inp: PyMarker) {
        let super_ = self_.as_mut();

        if let OverloadOptions::LabeledDashes {
            color: _color,
            label: _label,
        } = &mut super_.0
        {
            *_label = label_inp.0;
        } else {
            unreachable!()
        }
    }
}
#[derive(Debug, Clone)]
#[pyclass(name="MatchedDashes", extends=PyOverloadOptions)]
struct OverloadOptionsMatchedDashes;

#[pymethods]
impl OverloadOptionsMatchedDashes {
    #[new]
    fn new() -> (Self, PyOverloadOptions) {
        (Self, PyOverloadOptions(OverloadOptions::MatchedDashes {}))
    }
}