use hex_renderer::options::{EndPoint, Point, Marker};
use interface_macros::py_gen;

use super::{point::PyPoint, marker::PyMarker};

#[py_gen(remote = EndPoint)]
#[derive(Clone)]
pub enum PyEndPoint {
    Point(
        #[py_gen(name = "point", remote = PyPoint)]
        Point
    ),
    Match {
        radius: f32
    },
    BorderedMatch {
        match_radius: f32,
        #[py_gen(remote = PyMarker)]
        border: Marker,
    }
}