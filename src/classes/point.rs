use hex_renderer::options::Point;
use pyo3::{pyclass, PyResult, Py, pymethods, PyRef, PyRefMut, types::PyModule, Python};

use crate::initialize_subclass;

use super::marker::PyMarker;



pub fn initialize_module(py: Python, m: &PyModule) -> PyResult<()> {
    let point = PyModule::new(py, "Point")?;
    point.add_class::<PointNone>()?;
    point.add_class::<PointSingle>()?;
    point.add_class::<PointDouble>()?;
    m.add_submodule(point)?;

    Ok(())
}

/*#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Point {
    None,
    Single{marker: Marker},
    Double { inner: Marker, outer: Marker },
}*/
#[pyclass(subclass, name = "Point")]
#[derive(Clone, Debug)]
pub struct PyPoint(pub Point);

#[pymethods]
impl PyPoint {
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}
impl PyPoint {
    pub fn new_py(point: Point) -> PyResult<Py<Self>> {
        match point {
            Point::None => initialize_subclass(PointNone::new()),
            Point::Single(marker) => initialize_subclass(PointSingle::new(PyMarker(marker))),
            Point::Double { inner, outer } => {
                initialize_subclass(PointDouble::new(PyMarker(inner), PyMarker(outer)))
            }
        }
    }
}

#[derive(Clone, Debug)]
#[pyclass(extends=PyPoint, name="None_")]
struct PointNone;

#[pymethods]
impl PointNone {
    #[new]
    fn new() -> (Self, PyPoint) {
        (Self, PyPoint(Point::None))
    }
}

#[derive(Clone, Debug)]
#[pyclass(extends=PyPoint, name="Single")]
struct PointSingle;

#[pymethods]
impl PointSingle {
    #[new]
    fn new(marker: PyMarker) -> (Self, PyPoint) {
        (Self, PyPoint(Point::Single(marker.0)))
    }

    #[getter]
    fn get_marker(self_: PyRef<'_, Self>) -> PyMarker {
        let super_ = self_.as_ref();

        if let Point::Single(marker) = &super_.0 {
            PyMarker(*marker)
        } else {
            unreachable!()
        }
    }

    #[setter]
    fn set_marker(mut self_: PyRefMut<'_, Self>, marker: PyMarker) {
        let super_ = self_.as_mut();

        if let Point::Single(mark) = &mut super_.0 {
            *mark = marker.0;
        }
    }
}

//Double { inner: Marker, outer: Marker },
#[derive(Clone, Debug)]
#[pyclass(extends=PyPoint, name="Double")]
struct PointDouble;

#[pymethods]
impl PointDouble {
    #[new]
    fn new(inner: PyMarker, outer: PyMarker) -> (Self, PyPoint) {
        (
            Self,
            PyPoint(Point::Double {
                inner: inner.0,
                outer: outer.0,
            }),
        )
    }

    #[getter]
    fn get_inner(self_: PyRef<'_, Self>) -> PyMarker {
        let super_ = self_.as_ref();

        if let Point::Double { inner, outer: _ } = &super_.0 {
            PyMarker(*inner)
        } else {
            unreachable!()
        }
    }

    #[getter]
    fn get_outer(self_: PyRef<'_, Self>) -> PyMarker {
        let super_ = self_.as_ref();

        if let Point::Double { inner: _, outer } = &super_.0 {
            PyMarker(*outer)
        } else {
            unreachable!()
        }
    }

    #[setter]
    fn set_inner(mut self_: PyRefMut<'_, Self>, marker: PyMarker) {
        let super_ = self_.as_mut();

        if let Point::Double { inner, outer: _ } = &mut super_.0 {
            *inner = marker.0;
        }
    }

    #[setter]
    fn set_outer(mut self_: PyRefMut<'_, Self>, marker: PyMarker) {
        let super_ = self_.as_mut();

        if let Point::Double { inner: _, outer } = &mut super_.0 {
            *outer = marker.0;
        }
    }
}