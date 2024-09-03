use hex_renderer::options::Color;
use interface_macros::py_gen;
use pyo3::{types::PyModule, PyResult, Python};

pub fn add_class(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyColor>()?;

    Ok(())
}

#[py_gen(bridge = Color)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
///RGBA Color Class
pub struct PyColor(
    #[py_gen(name = "r")]
    ///Red (0-255)
    u8,
    #[py_gen(name = "g")] 
    ///Green (0-255)
    u8, 
    #[py_gen(name = "b")]
    ///Blue (0-255)
    u8, 
    #[py_gen(name = "a")]
    ///Alpha (0-255)
    u8
);
