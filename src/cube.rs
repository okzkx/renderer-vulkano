use bytemuck::{Pod, Zeroable};
use vulkano::impl_vertex;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Zeroable, Pod)]
pub struct Vertex {
    position: [f32; 3],
}

impl_vertex!(Vertex, position);

pub const VERTICES: [Vertex; 8] = [
    // back
    Vertex { position: [-0.5, -0.5, -0.5] },
    Vertex { position: [-0.5, 0.5, -0.5] },
    Vertex { position: [0.5, -0.5, -0.5] },
    Vertex { position: [0.5, 0.5, -0.5] },
    // front
    Vertex { position: [-0.5, -0.5, 0.5] },
    Vertex { position: [-0.5, 0.5, 0.5] },
    Vertex { position: [0.5, -0.5, 0.5] },
    Vertex { position: [0.5, 0.5, 0.5] },
];

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Zeroable, Pod)]
pub struct Normal {
    normal: [f32; 3],
}

impl_vertex!(Normal, normal);

pub const NORMALS: [Normal; 8] = [
    // back
    Normal { normal: [0.0, 0.0, -1.0] },
    Normal { normal: [0.0, 0.0, -1.0] },
    Normal { normal: [0.0, 0.0, -1.0] },
    Normal { normal: [0.0, 0.0, -1.0] },
    // front
    Normal { normal: [0.0, 0.0, 1.0] },
    Normal { normal: [0.0, 0.0, 1.0] },
    Normal { normal: [0.0, 0.0, 1.0] },
    Normal { normal: [0.0, 0.0, 1.0] },
];

pub const INDICES: [u16; 12] = [
    // back
    0 + 0, 0 + 2, 0 + 1, 0 + 1, 0 + 2, 0 + 3,
    // front
    4 + 0, 4 + 2, 4 + 1, 4 + 1, 4 + 2, 4 + 3,
];
