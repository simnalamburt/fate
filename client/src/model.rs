use std::io::BufReader;
use obj::*;
use resources;

pub fn load() -> ObjResult<(Vec<Vertex>, Vec<u16>)> {
    let res = try!(resources::load("dome.obj"));
    let obj = try!(load_obj(BufReader::new(res)));

    Ok((obj.vertices, obj.indices))
}
