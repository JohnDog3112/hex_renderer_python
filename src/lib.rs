mod classes;

use classes::{marker::PyMarker, color::PyColor, grid_options::PyGridOptions, pattern_variant::PyPatternVariant};
use hex_renderer::pattern_utils::Angle;
use pyo3::{prelude::*, PyClass};


/// A Python module implemented in Rust.
#[pymodule]
fn hex_renderer_python(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyColor>()?;
    m.add_class::<PyMarker>()?;

    classes::point::initialize_module(py, m)?;

    classes::end_point::initialize_module(py, m)?;

    classes::intersections::initialize_module(py, m)?;

    classes::triangle::initialize_module(py, m)?;

    classes::overload_options::initialize_module(py, m)?;

    classes::collision_option::initialize_module(py, m)?;

    classes::lines::initialize_module(py, m)?;

    classes::grid_pattern_options::initialize_module(py, m)?;
    

    m.add_class::<PyGridOptions>()?;
    m.add_class::<PyPatternVariant>()?;

    classes::grids::initialize_classes(m)?;

    Ok(())
}

fn initialize_subclass<T, G>(inp: (T, G)) -> PyResult<Py<G>>
where
    T: PyClass<BaseType = G>,
    G: PyClass<BaseType = PyAny>,
{
    let initializer = PyClassInitializer::from(inp.1).add_subclass(inp.0);

    Python::with_gil(|py| 
        Py::new(py, initializer)
            .map(|a| a.into_py(py))
            .map(|a| a.extract(py))?
    )
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
