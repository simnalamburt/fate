#![feature(io)]

extern crate glutin;
extern crate glium;

use std::old_io as io;

fn main() {
    use glium::DisplayBuild;

    let _display = glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title(format!("Hello world"))
        .build_glium().unwrap();

    println!("Press ENTER to quit");
    let _ = io::stdin().read_line();
}
