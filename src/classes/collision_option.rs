use hex_renderer::options::{Color, OverloadOptions, CollisionOption};
use interface_macros::py_gen;

use super::{color::PyColor, overload_options::PyOverloadOptions};

#[py_gen(remote = CollisionOption)]
#[derive(Clone)]
pub enum PyCollisionOption {
    Dashes(
        #[py_gen(name = "color", remote = PyColor)]
        Color
    ),
    MatchedDashes,
    ParallelLines,
    OverloadedParallel {
        max_line: usize,
        #[py_gen(remote = PyOverloadOptions)]
        overload: OverloadOptions
    }
}