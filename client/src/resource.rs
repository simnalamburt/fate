use bincode::rustc_serialize::{decode_from, DecodingResult};
use bincode::SizeLimit;
use obj::Obj;
use std::fs::File;
use std::path::{Path, PathBuf};

pub fn load_obj<T: AsRef<Path>>(name: T) -> DecodingResult<Obj> {
    let mut path = PathBuf::new();
    path.push(env!("OUT_DIR"));
    path.push(name);
    path.set_extension("obj.bin");

    let mut input = try!(File::open(path));
    let decoded: Obj = try!(decode_from(&mut input, SizeLimit::Infinite));
    Ok(decoded)
}
