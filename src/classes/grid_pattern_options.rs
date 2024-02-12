use hex_renderer::{options::{GridPatternOptions, Intersections, Lines}, pattern_utils::Angle};
use interface_macros::py_gen;

use super::{intersections::PyIntersections, lines::PyLines, angle_sig::AngleSig};

#[py_gen(remote = GridPatternOptions)]
#[derive(Clone)]
pub enum PyGridPatternOptions {
    Uniform (
        #[py_gen(name = "intersections", remote = PyIntersections)]
        Intersections,
        #[py_gen(name = "lines", remote = PyLines)]
        Lines
    ),
    Changing {
        #[py_gen(remote = Vec<(PyIntersections, PyLines)>)]
        variations: Vec<(Intersections, Lines)>,
        #[py_gen(remote = Vec<AngleSig>)]
        intros: Vec<Vec<Angle>>,
        #[py_gen(remote = Vec<AngleSig>)]
        retros: Vec<Vec<Angle>>
    }
}