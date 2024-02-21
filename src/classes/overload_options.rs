use hex_renderer::options::{OverloadOptions, Color, Marker};
use interface_macros::py_gen;

use super::{color::PyColor, marker::PyMarker};

#[py_gen(remote = OverloadOptions)]
#[derive(Clone)]
///Overload Options
pub enum PyOverloadOptions {
    Dashes(
        #[py_gen(name = "color", remote = PyColor)]
        Color
    ),
    LabeledDashes {
        #[py_gen(remote = PyColor)]
        color: Color,
        #[py_gen(remote = PyMarker)]
        label: Marker
    },
    MatchedDashes
}