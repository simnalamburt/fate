use std::io::Result;
use std::io::prelude::*;
use resources;

pub fn load() -> Result<(String, String)> {
    let mut vs = String::new();
    let mut fs = String::new();

    try!(try!(resources::load("vertex.glsl")).read_to_string(&mut vs));
    try!(try!(resources::load("fragment.glsl")).read_to_string(&mut fs));

    Ok((vs, fs))
}
