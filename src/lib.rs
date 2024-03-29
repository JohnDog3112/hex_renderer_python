use hex_renderer::pattern_utils::Angle;
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