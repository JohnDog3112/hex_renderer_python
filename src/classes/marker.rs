use hex_renderer::options::{Color, Marker};
use interface_macros::py_gen;

use super::color::PyColor;

#[py_gen(remote = Marker)]
#[derive(Clone)]
pub struct PyMarker {
    #[py_gen(remote = PyColor)]
    color: Color,
    radius: f32
}