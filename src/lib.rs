use hex_renderer::{pattern_utils::Angle, options::{Color, Marker, Point}};
use pyo3::{pymodule, Python, types::PyModule, PyResult};

mod classes;
#[pymodule]
fn hex_renderer_python(py: Python, m: &PyModule) -> PyResult<()> {
    
    classes::color::PyColor::add_class(py, m)?;
    classes::marker::PyMarker::add_class(py, m)?;
    classes::point::PyPoint::add_class(py, m)?;
    classes::end_point::PyEndPoint::add_class(py, m)?;
    classes::intersections::PyIntersections::add_class(py, m)?;
    classes::triangle::PyTriangle::add_class(py, m)?;
    classes::overload_options::PyOverloadOptions::add_class(py, m)?;
    classes::collision_option::PyCollisionOption::add_class(py, m)?;
    classes::lines::PyLines::add_class(py, m)?;

    m.add_class::<classes::angle_sig::AngleSig>()?;

    classes::grid_pattern_options::PyGridPatternOptions::add_class(py, m)?;
    classes::grid_options::PyGridOptions::add_class(py, m)?;

    m.add_class::<classes::pattern_variant::PyPatternVariant>()?;

    classes::grids::initialize_classes(m)?;

    Ok(())
}


#[allow(clippy::ptr_arg)]
fn angles_to_string(inp: &Vec<Angle>) -> String {
    inp.iter()
        .map(|angle| match angle {
            Angle::Forward => 'w',
            Angle::Right => 'e',
            Angle::BackRight => 'd',
            Angle::Back => 's',
            Angle::BackLeft => 'a',
            Angle::Left => 'q',
        })
        .collect()
}


/*#[::interface_macros::py_gen(remote = Color)]
#[derive(Clone)]
///comment!
struct PyColor(
    #[py_gen(name = "r")]
    ///R (0-255)
    u8,
    #[py_gen(name = "g")]
    ///G (0-255)
    u8,
    #[py_gen(name = "b")]
    ///B (0-255)
    u8,
    #[py_gen(name = "a")]
    ///A (0-255)
    /// does this work too?
    u8,
);

#[::interface_macros::py_gen(remote = Marker)]
#[derive(Clone)]

///a
struct PyMarker {
    ///b
    radius: f32,
    #[py_gen(remote = PyColor)]
    ///c
    color: Color,
}

#[::interface_macros::py_gen(remote = Point)]
#[derive(Clone)]
///a
pub enum PyPoint {
    ///None!
    None,
    ///Single
    Single(
        ///Marker in single
        #[py_gen(name = "marker", remote = PyMarker)]
        Marker
    ),
    ///Double
    Double { 
        #[py_gen(remote = PyMarker)]
        ///Inner marker
        inner: Marker,
        #[py_gen(remote = PyMarker)]
        ///outer marker
        outer: Marker 
    },
}*/


/*#[py_gen(remote = Color)]
#[derive(Clone)]
///Color!!
pub struct PyColor(
    #[py_gen(name = "r")]
    u8,
    #[py_gen(name = "g")] 
    u8, 
    #[py_gen(name = "b")]
    u8, 
    #[py_gen(name = "a")]
    u8
);*/


/*#[::interface_macros::py_type_gen]
#[::pyo3::pyclass]
#[derive(Clone)]
///this is a test
struct Color(u8, u8, u8, u8);

#[::interface_macros::py_type_gen]
#[::pyo3::pymethods]
impl Color {
    #[new]
    ///Creates a new color object from RGBA values
    /// :param r: Red (0-255)
    /// :param g: Green (0-255)
    /// :param b: Blue (0-255)
    /// :param a: Alpha/Opacity (0-255)
    fn new(r: u8, g: u8, b: u8, a: u8) -> Self{
        Self(r,g,b,a)
    }

    #[getter]
    ///Amount of red (0-255)
    fn get_r(&self) -> u8 {
        self.0
    }
    #[getter]
    ///Amount of green (0-255)
    fn get_g(&self) -> u8 {
        self.1
    }
    #[getter]
    ///Amount of blue (0-255)
    fn get_b(&self) -> u8 {
        self.2
    }
    #[getter]
    ///Amount of alpha/opacity (0-255)
    fn get_a(&self) -> u8 {
        self.3
    }

    fn with_all(&self, r: Option<u8>, g: Option<u8>, b: Option<u8>, a: Option<u8>) -> Self {
        let mut new = self.clone();

        if let Some(r) = r {
            new.0 = r;
        }

        if let Some(g) = g {
            new.1 = g;
        }

        if let Some(b) = b {
            new.2 = b;
        }

        if let Some(a) = a {
            new.3 = a;
        }

        new
    }
}*/

#[cfg(test)]
pub mod tests {
    use std::{fs::File, io::Write};



    #[test]
    fn print_stuffs() -> std::io::Result<()> {
        let types = ::interface_macros::collect_stored_types();
        let mut file = File::create("hex_renderer_python.pyi")?;
        file.write_all(types.as_bytes())?;
        Ok(())
    }
}
