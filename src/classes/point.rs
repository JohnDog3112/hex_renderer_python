use hex_renderer::options::{Point, Marker};
use interface_macros::py_gen;

use super::marker::PyMarker;

#[py_gen(remote = Point)]
#[derive(Clone)]
pub enum PyPoint {
    None,
    Single(
        #[py_gen(name = "marker", remote = PyMarker)]
        Marker
    ),
    Double { 
        #[py_gen(remote = PyMarker)]
        inner: Marker,
        #[py_gen(remote = PyMarker)]
        outer: Marker 
    },
}