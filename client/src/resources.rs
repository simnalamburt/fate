use std::path::Path;
use std::io::Result;
use std::env::current_exe;
use std::fs::File;

pub fn load<T: AsRef<Path>>(name: T) -> Result<File> {
    let exe = current_exe().unwrap();
    let dir = exe.parent().unwrap();

    let mut path = dir.to_path_buf();
    path.push("..");
    path.push("..");
    path.push("res");
    path.push(&name);

    File::open(&*path)
}
