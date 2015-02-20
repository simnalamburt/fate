use std::io::Result;

pub fn load() -> Result<(Vec<Vertex>, Vec<u16>)> {
    let vb = vec![
        Vertex::new(-0.5, 0.0, -10.0),
        Vertex::new(-0.5, 0.5, -10.0),
        Vertex::new( 0.5, 0.5, -10.0),
        Vertex::new( 0.9, 0.0, -10.0),
        Vertex::new( 0.8, 1.0, -10.0),
        Vertex::new(-0.5, 0.5, -10.0),
    ];

    let ib = vec![
        0, 1, 2,
        3, 4, 5 as u16
    ];

    Ok((vb, ib))
}

#[vertex_format]
#[derive(Copy)]
pub struct Vertex {
    position: [f32; 3],
}

impl Vertex {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Vertex { position: [x, y, z] }
    }
}
