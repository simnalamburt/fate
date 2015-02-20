use std::old_path::BytesContainer;
use std::fs::File;
use std::io::Result;
use std::env::current_exe;

pub fn load<T: BytesContainer>(name: T) -> Result<File> {
    let res = current_exe().unwrap().dir_path().join("..").join("res");

    File::open(&res.join(name))
}
