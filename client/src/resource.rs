use std::path::Path;
use std::io::Result;
use std::env::current_exe;
use std::fs::File;

pub fn load<T: AsRef<Path>>(name: T) -> Result<File> {
    let mut path = current_exe().unwrap();
    path.pop();
    path.pop();
    path.pop();
    path.push("res");
    path.push(name);

    File::open(path)
}
