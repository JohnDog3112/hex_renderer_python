use hex_renderer::options::{Color, Triangle, CollisionOption, Lines};
use interface_macros::py_gen;

use super::{color::PyColor, triangle::PyTriangle, collision_option::PyCollisionOption};

#[py_gen(remote = Lines)]
#[derive(Clone)]
pub enum PyLines {
    Monocolor {
        #[py_gen(remote = PyColor)]
        color: Color,
        bent: bool
    },
    Gradient {
        #[py_gen(remote = Vec<PyColor>)]
        colors: Vec<Color>,
        segments_per_color: usize,
        bent: bool
    },
    SegmentColors {
        #[py_gen(remote = Vec<PyColor>)]
        colors: Vec<Color>,
        #[py_gen(remote = PyTriangle)]
        triangles: Triangle,
        #[py_gen(remote = PyCollisionOption)]
        collisions: CollisionOption,
    }
}