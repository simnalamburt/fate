extern crate obj;
extern crate rustc_serialize;
extern crate bincode;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::env::current_exe;
use obj::{Obj, load_obj};
use bincode::rustc_serialize as bcode;
use bincode::SizeLimit;

fn main() {
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
    src.push("res");
    src.push("rilakkuma.obj");

    // Parse rilakkuma
    let input = BufReader::new(File::open(src).unwrap());
    let rilakkuma: Obj = load_obj(input).unwrap();

    // Serialize rilakkuma
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("rilakkuma.obj.bin");
    let mut output = File::create(&dest).unwrap();

    bcode::encode_into(&rilakkuma, &mut output, SizeLimit::Infinite).unwrap();
}
