use hex_renderer::options::Triangle;
use pyo3::{pyclass, pymethods, PyResult, Py, PyRef, PyRefMut, Python, types::PyModule};

use crate::initialize_subclass;

use super::marker::PyMarker;

/*pub enum Triangle {
    None,
    Match { radius: f32 },
    BorderMatch { match_radius: f32, border: Marker },
    BorderStartMatch { match_radius: f32, border: Marker },
}*/

pub fn initialize_module(py: Python, m: &PyModule) -> PyResult<()> {
    let triangles = PyModule::new(py, "Triangles")?;
    triangles.add_class::<TriangleNone>()?;
    triangles.add_class::<TriangleMatch>()?;
    triangles.add_class::<TriangleBorderMatch>()?;
    triangles.add_class::<TriangleBorderStartMatch>()?;
    m.add_submodule(triangles)?;
    Ok(())
}

#[derive(Debug, Clone)]
#[pyclass(name = "Triangle", subclass)]
pub struct PyTriangle(pub Triangle);

#[pymethods]
impl PyTriangle {
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

impl PyTriangle {
    pub fn new_py(triangle: Triangle) -> PyResult<Py<Self>> {
        match triangle {
            Triangle::None => initialize_subclass(TriangleNone::new()),
            Triangle::Match { radius } => initialize_subclass(TriangleMatch::new(radius)),
            Triangle::BorderMatch {
                match_radius,
                border,
            } => initialize_subclass(TriangleBorderMatch::new(match_radius, PyMarker(border))),
            Triangle::BorderStartMatch {
                match_radius,
                border,
            } => initialize_subclass(TriangleBorderStartMatch::new(
                match_radius,
                PyMarker(border),
            )),
        }
    }
}

#[derive(Debug, Clone)]
#[pyclass(name="None", extends=PyTriangle)]
struct TriangleNone;

#[pymethods]
impl TriangleNone {
    #[new]
    fn new() -> (Self, PyTriangle) {
        (Self, PyTriangle(Triangle::None {}))
    }
}
#[derive(Debug, Clone)]
#[pyclass(name="Match", extends=PyTriangle)]
struct TriangleMatch;

#[pymethods]
impl TriangleMatch {
    #[new]
    fn new(radius: f32) -> (Self, PyTriangle) {
        (Self, PyTriangle(Triangle::Match { radius }))
    }

    #[getter]
    fn get_radius(self_: PyRef<'_, Self>) -> f32 {
        let super_ = self_.as_ref();

        if let Triangle::Match { radius: _radius } = &super_.0 {
            *_radius
        } else {
            unreachable!()
        }
    }

    #[setter]
    fn set_radius(mut self_: PyRefMut<'_, Self>, radius_inp: f32) {
        let super_ = self_.as_mut();

        if let Triangle::Match { radius: _radius } = &mut super_.0 {
            *_radius = radius_inp;
        } else {
            unreachable!()
        }
    }
}
#[derive(Debug, Clone)]
#[pyclass(name="BorderMatch", extends=PyTriangle)]
struct TriangleBorderMatch;

#[pymethods]
impl TriangleBorderMatch {
    #[new]
    fn new(match_radius: f32, border: PyMarker) -> (Self, PyTriangle) {
        (
            Self,
            PyTriangle(Triangle::BorderMatch {
                match_radius,

                border: border.0,
            }),
        )
    }

    #[getter]
    fn get_match_radius(self_: PyRef<'_, Self>) -> f32 {
        let super_ = self_.as_ref();

        if let Triangle::BorderMatch {
            match_radius: _match_radius,
            border: _border,
        } = &super_.0
        {
            *_match_radius
        } else {
            unreachable!()
        }
    }

    #[setter]
    fn set_match_radius(mut self_: PyRefMut<'_, Self>, match_radius_inp: f32) {
        let super_ = self_.as_mut();

        if let Triangle::BorderMatch {
            match_radius: _match_radius,
            border: _border,
        } = &mut super_.0
        {
            *_match_radius = match_radius_inp;
        } else {
            unreachable!()
        }
    }
    #[getter]
    fn get_border(self_: PyRef<'_, Self>) -> PyMarker {
        let super_ = self_.as_ref();

        if let Triangle::BorderMatch {
            match_radius: _match_radius,
            border: _border,
        } = &super_.0
        {
            PyMarker(*_border)
        } else {
            unreachable!()
        }
    }

    #[setter]
    fn set_border(mut self_: PyRefMut<'_, Self>, border_inp: PyMarker) {
        let super_ = self_.as_mut();

        if let Triangle::BorderMatch {
            match_radius: _match_radius,
            border: _border,
        } = &mut super_.0
        {
            *_border = border_inp.0;
        } else {
            unreachable!()
        }
    }
}
#[derive(Debug, Clone)]
#[pyclass(name="BorderStartMatch", extends=PyTriangle)]
struct TriangleBorderStartMatch;

#[pymethods]
impl TriangleBorderStartMatch {
    #[new]
    fn new(match_radius: f32, border: PyMarker) -> (Self, PyTriangle) {
        (
            Self,
            PyTriangle(Triangle::BorderStartMatch {
                match_radius,

                border: border.0,
            }),
        )
    }

    #[getter]
    fn get_match_radius(self_: PyRef<'_, Self>) -> f32 {
        let super_ = self_.as_ref();

        if let Triangle::BorderStartMatch {
            match_radius: _match_radius,
            border: _border,
        } = &super_.0
        {
            *_match_radius
        } else {
            unreachable!()
        }
    }

    #[setter]
    fn set_match_radius(mut self_: PyRefMut<'_, Self>, match_radius_inp: f32) {
        let super_ = self_.as_mut();

        if let Triangle::BorderStartMatch {
            match_radius: _match_radius,
            border: _border,
        } = &mut super_.0
        {
            *_match_radius = match_radius_inp;
        } else {
            unreachable!()
        }
    }
    #[getter]
    fn get_border(self_: PyRef<'_, Self>) -> PyMarker {
        let super_ = self_.as_ref();

        if let Triangle::BorderStartMatch {
            match_radius: _match_radius,
            border: _border,
        } = &super_.0
        {
            PyMarker(*_border)
        } else {
            unreachable!()
        }
    }

    #[setter]
    fn set_border(mut self_: PyRefMut<'_, Self>, border_inp: PyMarker) {
        let super_ = self_.as_mut();

        if let Triangle::BorderStartMatch {
            match_radius: _match_radius,
            border: _border,
        } = &mut super_.0
        {
            *_border = border_inp.0;
        } else {
            unreachable!()
        }
    }
}