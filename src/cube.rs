use bytemuck::{Pod, Zeroable};
use vulkano::impl_vertex;


#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Zeroable, Pod)]
pub struct Vertex {
    position: [f32; 3],
}

impl_vertex!(Vertex, position);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Zeroable, Pod)]
pub struct Normal {
    normal: [f32; 3],
}

impl_vertex!(Normal, normal);


#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Zeroable, Pod)]
pub struct Float3 {
    value: [f32; 3],
}
// TODO: impl_vertex!(Vertex, position); Use for What


impl Float3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Float3 { value: [x, y, z] }
    }
}

//     4----- 6
//   / |    / |
// 0 --+- 2   |
// |   5--|-- 7
// | /    | /
// 1 ———— 3
const VERTICES_NO_REPEAT: [Float3; 8] = [
    Float3::new(-0.5, -0.5, -0.5),
    Float3::new(-0.5, 0.5, -0.5),
    Float3::new(0.5, -0.5, -0.5),
    Float3::new(0.5, 0.5, -0.5),
    Float3::new(-0.5, -0.5, 0.5),
    Float3::new(-0.5, 0.5, 0.5),
    Float3::new(0.5, -0.5, 0.5),
    Float3::new(0.5, 0.5, 0.5),
];


const NORMALS_NO_REPEAT: [Float3; 6] = [
    Float3::new(0.0, 0.0, -1.0),
    Float3::new(0.0, 0.0, 1.0),
    Float3::new(0.0, -1.0, 0.0),
    Float3::new(0.0, 1.0, 0.0),
    Float3::new(-1.0, 0.0, 0.0),
    Float3::new(1.0, 0.0, 0.0),
];

pub struct CubeModel {
    pub vertices: [Float3; 24],
    pub normals: [Float3; 24],
    pub indices: [u16; 6 * 2 * 3],
}

pub const CUBE: CubeModel = CubeModel {
    vertices: [
        // back
        VERTICES_NO_REPEAT[0],
        VERTICES_NO_REPEAT[1],
        VERTICES_NO_REPEAT[2],
        VERTICES_NO_REPEAT[3],
        // front
        VERTICES_NO_REPEAT[6],
        VERTICES_NO_REPEAT[7],
        VERTICES_NO_REPEAT[4],
        VERTICES_NO_REPEAT[5],
        // bottom
        VERTICES_NO_REPEAT[1],
        VERTICES_NO_REPEAT[5],
        VERTICES_NO_REPEAT[3],
        VERTICES_NO_REPEAT[7],
        // up
        VERTICES_NO_REPEAT[4],
        VERTICES_NO_REPEAT[5],
        VERTICES_NO_REPEAT[0],
        VERTICES_NO_REPEAT[1],
        // left
        VERTICES_NO_REPEAT[6],
        VERTICES_NO_REPEAT[7],
        VERTICES_NO_REPEAT[4],
        VERTICES_NO_REPEAT[5],
        // right
        VERTICES_NO_REPEAT[2],
        VERTICES_NO_REPEAT[3],
        VERTICES_NO_REPEAT[6],
        VERTICES_NO_REPEAT[7],
    ],
    normals: [
        // back
        NORMALS_NO_REPEAT[0],
        NORMALS_NO_REPEAT[0],
        NORMALS_NO_REPEAT[0],
        NORMALS_NO_REPEAT[0],
        // back
        NORMALS_NO_REPEAT[1],
        NORMALS_NO_REPEAT[1],
        NORMALS_NO_REPEAT[1],
        NORMALS_NO_REPEAT[1],
        // back
        NORMALS_NO_REPEAT[2],
        NORMALS_NO_REPEAT[2],
        NORMALS_NO_REPEAT[2],
        NORMALS_NO_REPEAT[2],
        // back
        NORMALS_NO_REPEAT[3],
        NORMALS_NO_REPEAT[3],
        NORMALS_NO_REPEAT[3],
        NORMALS_NO_REPEAT[3],
        // back
        NORMALS_NO_REPEAT[4],
        NORMALS_NO_REPEAT[4],
        NORMALS_NO_REPEAT[4],
        NORMALS_NO_REPEAT[4],
        // back
        NORMALS_NO_REPEAT[5],
        NORMALS_NO_REPEAT[5],
        NORMALS_NO_REPEAT[5],
        NORMALS_NO_REPEAT[5],
    ],
    indices: [
        // back
        0 + 4 * 0,
        1 + 4 * 0,
        2 + 4 * 0,
        2 + 4 * 0,
        1 + 4 * 0,
        3 + 4 * 0,
        // front
        0 + 4 * 1,
        1 + 4 * 1,
        2 + 4 * 1,
        2 + 4 * 1,
        1 + 4 * 1,
        3 + 4 * 1,
        // bottom
        0 + 4 * 2,
        1 + 4 * 2,
        2 + 4 * 2,
        2 + 4 * 2,
        1 + 4 * 2,
        3 + 4 * 2,
        // up
        0 + 4 * 3,
        1 + 4 * 3,
        2 + 4 * 3,
        2 + 4 * 3,
        1 + 4 * 3,
        3 + 4 * 3,
        // left
        0 + 4 * 4,
        1 + 4 * 4,
        2 + 4 * 4,
        2 + 4 * 4,
        1 + 4 * 4,
        3 + 4 * 4,
        // right
        0 + 4 * 5,
        1 + 4 * 5,
        2 + 4 * 5,
        2 + 4 * 5,
        1 + 4 * 5,
        3 + 4 * 5,
    ],
};
