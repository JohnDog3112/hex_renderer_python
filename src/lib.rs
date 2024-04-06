use hex_renderer::{pattern_utils::Angle};

use pyo3::{pymodule, Python, types::PyModule, PyResult};

pub mod classes;
#[pymodule]
fn hex_renderer_python(py: Python, m: &PyModule) -> PyResult<()> {
    
    classes::color::add_class(py, m)?;
    classes::marker::add_class(py, m)?;
    classes::point::add_class(py, m)?;
    classes::end_point::add_class(py, m)?;
    classes::intersections::add_class(py, m)?;
    classes::triangle::add_class(py, m)?;
    classes::overload_options::add_class(py, m)?;
    classes::collision_option::add_class(py, m)?;
    classes::lines::add_class(py, m)?;


    m.add_class::<classes::angle_sig::AngleSig>()?;

    classes::grid_pattern_options::add_class(py, m)?;
    classes::grid_options::add_class(py, m)?;

    m.add_class::<classes::pattern_variant::PyPatternVariant>()?;

    
    classes::grids::initialize_classes(m)?;

    test_fn(&<classes::color::PyColor as ::interface_macros::PyType>::path_string);

    Ok(())
}


/*fn test_fn<T: Fn() -> String + Send>(hi: T) -> String {
    hi()
}*/

/*fn test_fn(hi: &'static (dyn Send + Fn() -> String)) -> String {
    hi()
}*/

/*#[::interface_macros::py_type_gen(nested = ModuleTest)]
#[pyo3::pyclass]
struct ItemOne;
#[::interface_macros::py_type_gen(nested = ModuleTest)]
#[pyo3::pyclass]
struct ItemTwo;
#[::interface_macros::py_type_gen(nested = ModuleTest)]
#[pyo3::pyclass]
struct ItemThree;


#[::interface_macros::py_type_gen(module = "ModuleTest", nested = ModuleTestTwo, union = ("hello", [ItemOne, ItemTwo, ItemThree]))]
struct ModuleTest;

#[::interface_macros::py_type_gen(module = "ModuleTestTwo")]
struct ModuleTestTwo;*/


/*#[::pyo3::pyclass]
#[derive(Debug, PartialEq, PartialOrd)]
struct Test {
    f: f32
}

#[::pyo3::pymethods]
impl Test {
    fn __repr__(&self) -> String {
        format!("{:?}", self)
    }

    fn __richcmp__(&self, other: &Self, op: pyo3::basic::CompareOp) -> PyResult<bool> {
        Ok(match op {
            pyo3::pyclass::CompareOp::Lt => self < other,
            pyo3::pyclass::CompareOp::Le => self <= other,
            pyo3::pyclass::CompareOp::Eq => self == other,
            pyo3::pyclass::CompareOp::Ne => self != other,
            pyo3::pyclass::CompareOp::Gt => self > other,
            pyo3::pyclass::CompareOp::Ge => self >= other,
        })
    }
}*/


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

#[cfg(test)]
pub mod tests {
    use std::{fs::File, io::Write};

    const INIT_PY: &str = "
from .hex_renderer_python import *

__doc__ = hex_renderer_python.__doc__
if hasattr(hex_renderer_python, \"__all__\"):
    __all__ = hex_renderer_python.__all__

";

    #[test]
    fn print_stuffs() -> std::io::Result<()> {
        let (types, declarations) = ::interface_macros::collect_stored_types();
        let mut file = File::create("hex_renderer_python/__init__.pyi")?;
        file.write_all(types.as_bytes())?;

        let mut init_file = File::create("hex_renderer_python/__init__.py")?;
        init_file.write_all(INIT_PY.as_bytes())?;
        init_file.write_all(declarations.as_bytes())?;

        Ok(())
    }
}