use hex_renderer::options::{Point, Intersections, EndPoint};
use interface_macros::py_gen;

use super::{point::PyPoint, end_point::PyEndPoint};

#[py_gen(remote = Intersections)]
#[derive(Clone)]
pub enum PyIntersections {
    Nothing,
    UniformPoints(
        #[py_gen(name = "point", remote = PyPoint)]
        Point,
    ),
    EndsAndMiddle {
        #[py_gen(remote = PyEndPoint)]
        start: EndPoint,
        #[py_gen(remote = PyPoint)]
        middle: Point,
        #[py_gen(remote = PyEndPoint)]
        end: EndPoint
    }
}