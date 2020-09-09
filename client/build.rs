extern crate bincode;
extern crate obj;
extern crate rustc_serialize;
extern crate xz2;

use bincode::rustc_serialize as bcode;
use bincode::SizeLimit;
use obj::{load_obj, Obj};
use std::env;
use std::env::current_exe;
use std::fs::{metadata, File};
use std::io::BufReader;
use std::path::Path;
use xz2::read::XzDecoder;

fn main() {
    // Check if the rilakkuma has already packed
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("rilakkuma.obj.bin");

    let meta = metadata(&dest);
    let exists = if let Ok(meta) = meta {
        meta.is_file()
    } else {
        false
    };

    // Perform resource packing only when rilakkuma.obj.bin doesn't exist
    if exists {
        return;
    }

    // As of rust 1.2, `current_exe()` of build script looks like below:
    //
    //     /fate/client/target/debug/build/client-057d32d9862c7834/build-script-build
    //
    // So `current_exe()/../../../../../res` will return desired resource's path
    let mut src = current_exe().unwrap();
    src.pop();
    src.pop();
    src.pop();
    src.pop();
    src.pop();
    src.push("client");
    src.push("res");
    src.push("rilakkuma.obj.xz");

    // Parse rilakkuma
    let input = XzDecoder::new(File::open(src).unwrap());
    let input = BufReader::new(input);
    let rilakkuma: Obj = load_obj(input).unwrap();

    // Serialize rilakkuma
    let mut output = File::create(&dest).unwrap();

    bcode::encode_into(&rilakkuma, &mut output, SizeLimit::Infinite).unwrap();
}
