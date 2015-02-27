use std::path::AsPath;
use std::io::Result;
use std::env::current_exe;
use std::fs::File;

pub fn load<T: AsPath>(name: T) -> Result<File> {
    let exe = current_exe().unwrap();
    let dir = exe.parent().unwrap();

    let mut path = dir.to_path_buf();
    path.push("..");
    path.push("res");
    path.push(&name);

    File::open(&*path)
}
