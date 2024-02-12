use hex_renderer::options::Color;
use interface_macros::py_gen;

#[py_gen(remote = Color)]
#[derive(Clone)]
pub struct PyColor(
    #[py_gen(name = "r")]
    u8,
    #[py_gen(name = "g")] 
    u8, 
    #[py_gen(name = "b")]
    u8, 
    #[py_gen(name = "a")]
    u8
);