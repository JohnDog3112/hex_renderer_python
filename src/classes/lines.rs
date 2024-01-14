use hex_renderer::options::Lines;
use pyo3::{pyclass, pymethods, PyResult, Py, PyRef, PyRefMut, Python, types::PyModule};

use crate::initialize_subclass;

use super::{color::PyColor, collision_option::PyCollisionOption, triangle::PyTriangle};

/*pub enum Lines {
    Monocolor {
        color: Color,
        bent: bool,
    },
    Gradient {
        colors: Vec<Color>,
        segments_per_color: usize,
        bent: bool,
    },
    SegmentColors {
        colors: Vec<Color>,
        triangles: Triangle,
        collisions: CollisionOption,
    },
}*/

pub fn initialize_module(py: Python, m: &PyModule) -> PyResult<()> {
    let lines = PyModule::new(py, "Lines")?;
    lines.add_class::<LinesMonocolor>()?;
    lines.add_class::<LinesGradient>()?;
    lines.add_class::<LinesSegmentColors>()?;
    m.add_submodule(lines)?;

    Ok(())
}

#[derive(Debug, Clone)]
#[pyclass(name = "Lines", subclass)]
pub struct PyLines(pub Lines);

#[pymethods]
impl PyLines {
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

impl PyLines {
    pub fn new_py(lines: Lines) -> PyResult<Py<Self>> {
        match lines {
            Lines::Monocolor { color, bent } => {
                initialize_subclass(LinesMonocolor::new(PyColor(color), bent))
            }
            Lines::Gradient {
                colors,
                segments_per_color,
                bent,
            } => initialize_subclass(LinesGradient::new(
                colors.into_iter().map(PyColor).collect(),
                segments_per_color,
                bent,
            )),
            Lines::SegmentColors {
                colors,
                triangles,
                collisions,
            } => initialize_subclass(LinesSegmentColors::new(
                colors.into_iter().map(PyColor).collect(),
                PyTriangle(triangles),
                PyCollisionOption(collisions),
            )),
        }
    }
}

#[derive(Debug, Clone)]
#[pyclass(name="Monocolor", extends=PyLines)]
struct LinesMonocolor;

#[pymethods]
impl LinesMonocolor {
    #[new]
    fn new(color: PyColor, bent: bool) -> (Self, PyLines) {
        (
            Self,
            PyLines(Lines::Monocolor {
                color: color.0,

                bent,
            }),
        )
    }

    #[getter]
    fn get_color(self_: PyRef<'_, Self>) -> PyColor {
        let super_ = self_.as_ref();

        if let Lines::Monocolor {
            color: _color,
            bent: _bent,
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

        if let Lines::Monocolor {
            color: _color,
            bent: _bent,
        } = &mut super_.0
        {
            *_color = color_inp.0;
        } else {
            unreachable!()
        }
    }
    #[getter]
    fn get_bent(self_: PyRef<'_, Self>) -> bool {
        let super_ = self_.as_ref();

        if let Lines::Monocolor {
            color: _color,
            bent: _bent,
        } = &super_.0
        {
            *_bent
        } else {
            unreachable!()
        }
    }

    #[setter]
    fn set_bent(mut self_: PyRefMut<'_, Self>, bent_inp: bool) {
        let super_ = self_.as_mut();

        if let Lines::Monocolor {
            color: _color,
            bent: _bent,
        } = &mut super_.0
        {
            *_bent = bent_inp;
        } else {
            unreachable!()
        }
    }
}
#[derive(Debug, Clone)]
#[pyclass(name="Gradient", extends=PyLines)]
struct LinesGradient;

#[pymethods]
impl LinesGradient {
    #[new]
    fn new(colors: Vec<PyColor>, segments_per_color: usize, bent: bool) -> (Self, PyLines) {
        (
            Self,
            PyLines(Lines::Gradient {
                colors: colors.into_iter().map(|a| a.0).collect(),

                segments_per_color,

                bent,
            }),
        )
    }

    #[getter]
    fn get_colors(self_: PyRef<'_, Self>) -> Vec<PyColor> {
        let super_ = self_.as_ref();

        if let Lines::Gradient {
            colors: _colors,
            segments_per_color: _segments_per_color,
            bent: _bent,
        } = &super_.0
        {
            _colors.iter().map(|a| PyColor(*a)).collect()
        } else {
            unreachable!()
        }
    }

    #[setter]
    fn set_colors(mut self_: PyRefMut<'_, Self>, colors_inp: Vec<PyColor>) {
        let super_ = self_.as_mut();

        if let Lines::Gradient {
            colors: _colors,
            segments_per_color: _segments_per_color,
            bent: _bent,
        } = &mut super_.0
        {
            *_colors = colors_inp.into_iter().map(|a| a.0).collect();
        } else {
            unreachable!()
        }
    }
    #[getter]
    fn get_segments_per_color(self_: PyRef<'_, Self>) -> usize {
        let super_ = self_.as_ref();

        if let Lines::Gradient {
            colors: _colors,
            segments_per_color: _segments_per_color,
            bent: _bent,
        } = &super_.0
        {
            *_segments_per_color
        } else {
            unreachable!()
        }
    }

    #[setter]
    fn set_segments_per_color(mut self_: PyRefMut<'_, Self>, segments_per_color_inp: usize) {
        let super_ = self_.as_mut();

        if let Lines::Gradient {
            colors: _colors,
            segments_per_color: _segments_per_color,
            bent: _bent,
        } = &mut super_.0
        {
            *_segments_per_color = segments_per_color_inp;
        } else {
            unreachable!()
        }
    }
    #[getter]
    fn get_bent(self_: PyRef<'_, Self>) -> bool {
        let super_ = self_.as_ref();

        if let Lines::Gradient {
            colors: _colors,
            segments_per_color: _segments_per_color,
            bent: _bent,
        } = &super_.0
        {
            *_bent
        } else {
            unreachable!()
        }
    }

    #[setter]
    fn set_bent(mut self_: PyRefMut<'_, Self>, bent_inp: bool) {
        let super_ = self_.as_mut();

        if let Lines::Gradient {
            colors: _colors,
            segments_per_color: _segments_per_color,
            bent: _bent,
        } = &mut super_.0
        {
            *_bent = bent_inp;
        } else {
            unreachable!()
        }
    }
}
#[derive(Debug, Clone)]
#[pyclass(name="SegmentColors", extends=PyLines)]
struct LinesSegmentColors;

#[pymethods]
impl LinesSegmentColors {
    #[new]
    fn new(
        colors: Vec<PyColor>,
        triangles: PyTriangle,
        collisions: PyCollisionOption,
    ) -> (Self, PyLines) {
        (
            Self,
            PyLines(Lines::SegmentColors {
                colors: colors.into_iter().map(|a| a.0).collect(),

                triangles: triangles.0,

                collisions: collisions.0,
            }),
        )
    }

    #[getter]
    fn get_colors(self_: PyRef<'_, Self>) -> Vec<PyColor> {
        let super_ = self_.as_ref();

        if let Lines::SegmentColors {
            colors: _colors,
            triangles: _triangles,
            collisions: _collisions,
        } = &super_.0
        {
            _colors.iter().map(|a| PyColor(*a)).collect()
        } else {
            unreachable!()
        }
    }

    #[setter]
    fn set_colors(mut self_: PyRefMut<'_, Self>, colors_inp: Vec<PyColor>) {
        let super_ = self_.as_mut();

        if let Lines::SegmentColors {
            colors: _colors,
            triangles: _triangles,
            collisions: _collisions,
        } = &mut super_.0
        {
            *_colors = colors_inp.into_iter().map(|a| a.0).collect();
        } else {
            unreachable!()
        }
    }
    #[getter]
    fn get_triangles(self_: PyRef<'_, Self>) -> PyResult<Py<PyTriangle>> {
        let super_ = self_.as_ref();

        if let Lines::SegmentColors {
            colors: _colors,
            triangles: _triangles,
            collisions: _collisions,
        } = &super_.0
        {
            PyTriangle::new_py(*_triangles)
        } else {
            unreachable!()
        }
    }

    #[setter]
    fn set_triangles(mut self_: PyRefMut<'_, Self>, triangles_inp: PyTriangle) {
        let super_ = self_.as_mut();

        if let Lines::SegmentColors {
            colors: _colors,
            triangles: _triangles,
            collisions: _collisions,
        } = &mut super_.0
        {
            *_triangles = triangles_inp.0;
        } else {
            unreachable!()
        }
    }
    #[getter]
    fn get_collisions(self_: PyRef<'_, Self>) -> PyResult<Py<PyCollisionOption>> {
        let super_ = self_.as_ref();

        if let Lines::SegmentColors {
            colors: _colors,
            triangles: _triangles,
            collisions: _collisions,
        } = &super_.0
        {
            PyCollisionOption::new_py(*_collisions)
        } else {
            unreachable!()
        }
    }

    #[setter]
    fn set_collisions(mut self_: PyRefMut<'_, Self>, collisions_inp: PyCollisionOption) {
        let super_ = self_.as_mut();

        if let Lines::SegmentColors {
            colors: _colors,
            triangles: _triangles,
            collisions: _collisions,
        } = &mut super_.0
        {
            *_collisions = collisions_inp.0;
        } else {
            unreachable!()
        }
    }
}