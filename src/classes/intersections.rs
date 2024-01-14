use hex_renderer::options::Intersections;
use pyo3::{pyclass, pymethods, PyResult, Py, PyRef, PyRefMut, Python, types::PyModule};

use crate::initialize_subclass;

use super::{end_point::PyEndPoint, point::PyPoint};


/*pub enum Intersections {
    Nothing,
    UniformPoints{point: Point},
    EndsAndMiddle {
        start: EndPoint,
        end: EndPoint,
        middle: Point,
    },
}*/

pub fn initialize_module(py: Python, m: &PyModule) -> PyResult<()> {
    let intersections = PyModule::new(py, "Intersections")?;
    intersections.add_class::<IntersectionsNothing>()?;
    intersections.add_class::<IntersectionsUniformPoints>()?;
    intersections.add_class::<IntersectionsEndsAndMiddle>()?;
    m.add_submodule(intersections)?;

    Ok(())
}

#[derive(Clone, Debug)]
#[pyclass(name = "Intersections", subclass)]
pub struct PyIntersections(pub Intersections);

#[pymethods]
impl PyIntersections {
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

impl PyIntersections {
    pub fn new_py(intersection: Intersections) -> PyResult<Py<Self>> {
        match intersection {
            Intersections::Nothing => initialize_subclass(IntersectionsNothing::new()),
            Intersections::UniformPoints(point) => {
                initialize_subclass(IntersectionsUniformPoints::new(PyPoint(point)))
            }
            Intersections::EndsAndMiddle { start, end, middle } => {
                initialize_subclass(IntersectionsEndsAndMiddle::new(
                    PyEndPoint(start),
                    PyPoint(middle),
                    PyEndPoint(end),
                ))
            }
        }
    }
}

#[derive(Clone, Debug)]
#[pyclass(extends=PyIntersections, name="Nothing")]
struct IntersectionsNothing;

//Nothing
#[pymethods]
impl IntersectionsNothing {
    #[new]
    fn new() -> (Self, PyIntersections) {
        (Self, PyIntersections(Intersections::Nothing))
    }
}

//UniformPoints{point: Point},
#[derive(Clone, Debug)]
#[pyclass(extends=PyIntersections, name="UniformPoints")]
struct IntersectionsUniformPoints;

#[pymethods]
impl IntersectionsUniformPoints {
    #[new]
    fn new(point: PyPoint) -> (Self, PyIntersections) {
        (Self, PyIntersections(Intersections::UniformPoints(point.0)))
    }

    #[getter]
    fn get_point(self_: PyRef<'_, Self>) -> PyResult<Py<PyPoint>> {
        let super_ = self_.as_ref();

        if let Intersections::UniformPoints(point) = super_.0 {
            PyPoint::new_py(point)
        } else {
            unreachable!()
        }
    }

    #[setter]
    fn set_point(mut self_: PyRefMut<'_, Self>, point: PyPoint) {
        let super_ = self_.as_mut();

        if let Intersections::UniformPoints(poin) = &mut super_.0 {
            *poin = point.0
        } else {
            unreachable!()
        }
    }
}

/*
EndsAndMiddle {
    start: EndPoint,
    end: EndPoint,
    middle: Point,
},
*/
#[derive(Clone, Debug)]
#[pyclass(extends=PyIntersections, name="EndsAndMiddle")]
struct IntersectionsEndsAndMiddle;

#[pymethods]
impl IntersectionsEndsAndMiddle {
    #[new]
    fn new(start: PyEndPoint, middle: PyPoint, end: PyEndPoint) -> (Self, PyIntersections) {
        (
            Self,
            PyIntersections(Intersections::EndsAndMiddle {
                start: start.0,
                middle: middle.0,
                end: end.0,
            }),
        )
    }

    #[getter]
    fn get_start(self_: PyRef<'_, Self>) -> PyResult<Py<PyEndPoint>> {
        let super_ = self_.as_ref();

        if let Intersections::EndsAndMiddle {
            start: point,
            end: _,
            middle: _,
        } = &super_.0
        {
            PyEndPoint::new_py(*point)
        } else {
            unreachable!()
        }
    }

    #[getter]
    fn get_middle(self_: PyRef<'_, Self>) -> PyResult<Py<PyPoint>> {
        let super_ = self_.as_ref();

        if let Intersections::EndsAndMiddle {
            start: _,
            end: _,
            middle: point,
        } = &super_.0
        {
            PyPoint::new_py(*point)
        } else {
            unreachable!()
        }
    }

    #[getter]
    fn get_end(self_: PyRef<'_, Self>) -> PyResult<Py<PyEndPoint>> {
        let super_ = self_.as_ref();

        if let Intersections::EndsAndMiddle {
            start: _,
            end: point,
            middle: _,
        } = &super_.0
        {
            PyEndPoint::new_py(*point)
        } else {
            unreachable!()
        }
    }

    #[setter]
    fn set_start(mut self_: PyRefMut<'_, Self>, point: PyEndPoint) {
        let super_ = self_.as_mut();

        if let Intersections::EndsAndMiddle {
            start: poin,
            end: _,
            middle: _,
        } = &mut super_.0
        {
            *poin = point.0;
        }
    }

    #[setter]
    fn set_middle(mut self_: PyRefMut<'_, Self>, point: PyPoint) {
        let super_ = self_.as_mut();

        if let Intersections::EndsAndMiddle {
            start: _,
            end: _,
            middle: poin,
        } = &mut super_.0
        {
            *poin = point.0;
        }
    }

    #[setter]
    fn set_end(mut self_: PyRefMut<'_, Self>, point: PyEndPoint) {
        let super_ = self_.as_mut();

        if let Intersections::EndsAndMiddle {
            start: _,
            end: poin,
            middle: _,
        } = &mut super_.0
        {
            *poin = point.0;
        }
    }
}