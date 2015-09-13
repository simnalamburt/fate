use std::path::{Path, PathBuf};
use std::fs::File;
use obj::Obj;
use bincode::SizeLimit;
use bincode::rustc_serialize::{decode_from, DecodingResult};

pub fn load_obj<T: AsRef<Path>>(name: T) -> DecodingResult<Obj> {
    let mut path = PathBuf::new();
    path.push(env!("OUT_DIR"));
    path.push(name);
    path.set_extension("obj.bin");

    let mut input = try!(File::open(path));
    let decoded: Obj = try!(decode_from(&mut input, SizeLimit::Infinite));
    Ok(decoded)
}
