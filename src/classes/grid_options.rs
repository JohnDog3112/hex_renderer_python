use hex_renderer::options::{GridPatternOptions, Point, GridOptions};
use interface_macros::py_gen;

use super::{grid_pattern_options::PyGridPatternOptions, point::PyPoint};

#[py_gen(remote = GridOptions)]
#[derive(Clone)]
pub struct PyGridOptions {
    pub line_thickness: f32,

    #[py_gen(remote = PyGridPatternOptions)]
    pub pattern_options: GridPatternOptions,

    #[py_gen(remote = PyPoint)]
    pub center_dot: Point,
}