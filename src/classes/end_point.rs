use hex_renderer::options::EndPoint;
use pyo3::{pyclass, pymethods, PyResult, Py, PyRef, PyRefMut, types::PyModule, Python};

use crate::initialize_subclass;

use super::{point::PyPoint, marker::PyMarker};



pub fn initialize_module(py: Python, m: &PyModule) -> PyResult<()> {
    let end_point = PyModule::new(py, "EndPoint")?;
    end_point.add_class::<EndPointPoint>()?;
    end_point.add_class::<EndPointMatch>()?;
    end_point.add_class::<EndPointBorderedMatch>()?;
    m.add_submodule(end_point)?;

    Ok(())
}
/*pub enum EndPoint {
    Point{ point: Point},
    Match { radius: f32 },
    BorderedMatch { match_radius: f32, border: Marker },
}*/
#[derive(Clone, Debug)]
#[pyclass(subclass, name = "EndPoint")]
pub struct PyEndPoint(pub EndPoint);

#[pymethods]
impl PyEndPoint {
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}
impl PyEndPoint {
    pub fn new_py(point: EndPoint) -> PyResult<Py<Self>> {
        match point {
            EndPoint::Point(point) => initialize_subclass(EndPointPoint::new(PyPoint(point))),
            EndPoint::Match { radius } => initialize_subclass(EndPointMatch::new(radius)),
            EndPoint::BorderedMatch {
                match_radius,
                border,
            } => initialize_subclass(EndPointBorderedMatch::new(match_radius, PyMarker(border))),
        }
    }
}

#[derive(Clone, Debug)]
#[pyclass(extends=PyEndPoint, name="Point")]
struct EndPointPoint;

//Point{ point: Point},
#[pymethods]
impl EndPointPoint {
    #[new]
    fn new(point: PyPoint) -> (Self, PyEndPoint) {
        (Self, PyEndPoint(EndPoint::Point(point.0)))
    }

    #[getter]
    fn get_point(self_: PyRef<'_, Self>) -> PyResult<Py<PyPoint>> {
        let super_ = self_.as_ref();

        if let EndPoint::Point(point) = &super_.0 {
            PyPoint::new_py(*point)
        } else {
            unreachable!()
        }
    }

    #[setter]
    fn set_point(mut self_: PyRefMut<'_, Self>, point: PyPoint) {
        let super_ = self_.as_mut();

        if let EndPoint::Point(poin) = &mut super_.0 {
            *poin = point.0;
        }
    }
}
#[derive(Clone, Debug)]
#[pyclass(extends=PyEndPoint, name="Match")]
struct EndPointMatch;

//Match { radius: f32 },
#[pymethods]
impl EndPointMatch {
    #[new]
    fn new(radius: f32) -> (Self, PyEndPoint) {
        (Self, PyEndPoint(EndPoint::Match { radius }))
    }

    #[getter]
    fn get_radius(self_: PyRef<'_, Self>) -> f32 {
        let super_ = self_.as_ref();

        if let EndPoint::Match { radius } = super_.0 {
            radius
        } else {
            unreachable!()
        }
    }

    #[setter]
    fn set_radius(mut self_: PyRefMut<'_, Self>, radius: f32) {
        let super_ = self_.as_mut();

        if let EndPoint::Match { radius: rad } = &mut super_.0 {
            *rad = radius;
        }
    }
}

//BorderedMatch { match_radius: f32, border: Marker },
#[derive(Clone, Debug)]
#[pyclass(extends=PyEndPoint, name="BorderedMatch")]
struct EndPointBorderedMatch;

#[pymethods]
impl EndPointBorderedMatch {
    #[new]
    fn new(match_radius: f32, border: PyMarker) -> (Self, PyEndPoint) {
        (
            Self,
            PyEndPoint(EndPoint::BorderedMatch {
                match_radius,
                border: border.0,
            }),
        )
    }

    #[getter]
    fn get_match_radius(self_: PyRef<'_, Self>) -> f32 {
        let super_ = self_.as_ref();

        if let EndPoint::BorderedMatch {
            match_radius,
            border: _,
        } = super_.0
        {
            match_radius
        } else {
            unreachable!()
        }
    }

    #[setter]
    fn set_match_radius(mut self_: PyRefMut<'_, Self>, match_radius: f32) {
        let super_ = self_.as_mut();

        if let EndPoint::BorderedMatch {
            match_radius: radius,
            border: _,
        } = &mut super_.0
        {
            *radius = match_radius;
        } else {
            unreachable!()
        }
    }

    #[getter]
    fn get_border(self_: PyRef<'_, Self>) -> PyMarker {
        let super_ = self_.as_ref();

        if let EndPoint::BorderedMatch {
            match_radius: _,
            border,
        } = &super_.0
        {
            PyMarker(*border)
        } else {
            unreachable!()
        }
    }

    #[setter]
    fn set_border(mut self_: PyRefMut<'_, Self>, border: PyMarker) {
        let super_ = self_.as_mut();

        if let EndPoint::BorderedMatch {
            match_radius: _,
            border: mark,
        } = &mut super_.0
        {
            *mark = border.0;
        } else {
            unreachable!()
        }
    }
}