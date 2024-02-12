use hex_renderer::options::{Triangle, Marker};
use interface_macros::py_gen;

use super::marker::PyMarker;

#[py_gen(remote = Triangle)]
#[derive(Clone)]
pub enum PyTriangle {
    None,
    Match {
        radius: f32
    },
    BorderMatch {
        match_radius: f32,
        #[py_gen(remote = PyMarker)]
        border: Marker
    },
    BorderStartMatch {
        match_radius: f32,
        #[py_gen(remote = PyMarker)]
        border: Marker
    }
}