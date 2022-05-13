use bytemuck::{Pod, Zeroable};
use vulkano::impl_vertex;


#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Zeroable, Pod)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    coords: [f32; 2],
}

impl_vertex!(Vertex, position, normal, coords);
impl Vertex {
    #[allow(dead_code)]
    fn new(position: [f32; 3], normal: [f32; 3]) -> Self {
        Vertex { position, normal, coords: [0.0, 0.0] }
    }
}

//     4----- 6
//   / |    / |
// 0 --+- 2   |
// |   5--|-- 7
// | /    | /
// 1 ———— 3
// const POSITIONS_NO_REPEAT: [[f32; 3]; 8] = [
//     [-0.5, -0.5, -0.5],
//     [-0.5, 0.5, -0.5],
//     [0.5, -0.5, -0.5],
//     [0.5, 0.5, -0.5],
//     [-0.5, -0.5, 0.5],
//     [-0.5, 0.5, 0.5],
//     [0.5, -0.5, 0.5],
//     [0.5, 0.5, 0.5],
// ];

const POSITIONS_NO_REPEAT: [[f32; 3]; 8] = [
    [-0.5, 0.5, -0.5],
    [-0.5, -0.5, -0.5],
    [0.5, 0.5, -0.5],
    [0.5, -0.5, -0.5],
    [-0.5, 0.5, 0.5],
    [-0.5, -0.5, 0.5],
    [0.5, 0.5, 0.5],
    [0.5, -0.5, 0.5],
];


const NORMALS_NO_REPEAT: [[f32; 3]; 6] = [
    [0.0, 0.0, -1.0],
    [0.0, 0.0, 1.0],
    [0.0, -1.0, 0.0],
    [0.0, 1.0, 0.0],
    [-1.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],
];

pub struct CubeModel {
    pub vertices: [Vertex; 24],
    pub indices: [u16; 6 * 2 * 3],
}

pub struct CoordsMode {
    left_top: [f32; 2],
    left_bottom: [f32; 2],
    right_top: [f32; 2],
    right_bottom: [f32; 2],
}

pub const COORDS: CoordsMode = CoordsMode {
    left_top: [0.0, 1.0],
    left_bottom: [0.0, 0.0],
    right_top: [1.0, 1.0],
    right_bottom: [1.0, 0.0],
};

pub const CUBE: CubeModel = CubeModel {
    vertices: [
        Vertex { position: POSITIONS_NO_REPEAT[0], normal: NORMALS_NO_REPEAT[0], coords: COORDS.left_top },
        Vertex { position: POSITIONS_NO_REPEAT[1], normal: NORMALS_NO_REPEAT[0], coords: COORDS.left_bottom },
        Vertex { position: POSITIONS_NO_REPEAT[2], normal: NORMALS_NO_REPEAT[0], coords: COORDS.right_top },
        Vertex { position: POSITIONS_NO_REPEAT[3], normal: NORMALS_NO_REPEAT[0], coords: COORDS.right_bottom },
        Vertex { position: POSITIONS_NO_REPEAT[6], normal: NORMALS_NO_REPEAT[1], coords: COORDS.left_top },
        Vertex { position: POSITIONS_NO_REPEAT[7], normal: NORMALS_NO_REPEAT[1], coords: COORDS.left_bottom },
        Vertex { position: POSITIONS_NO_REPEAT[4], normal: NORMALS_NO_REPEAT[1], coords: COORDS.right_top },
        Vertex { position: POSITIONS_NO_REPEAT[5], normal: NORMALS_NO_REPEAT[1], coords: COORDS.right_bottom },
        Vertex { position: POSITIONS_NO_REPEAT[1], normal: NORMALS_NO_REPEAT[2], coords: COORDS.left_top },
        Vertex { position: POSITIONS_NO_REPEAT[5], normal: NORMALS_NO_REPEAT[2], coords: COORDS.left_bottom },
        Vertex { position: POSITIONS_NO_REPEAT[3], normal: NORMALS_NO_REPEAT[2], coords: COORDS.right_top },
        Vertex { position: POSITIONS_NO_REPEAT[7], normal: NORMALS_NO_REPEAT[2], coords: COORDS.right_bottom },
        Vertex { position: POSITIONS_NO_REPEAT[4], normal: NORMALS_NO_REPEAT[3], coords: COORDS.left_top },
        Vertex { position: POSITIONS_NO_REPEAT[5], normal: NORMALS_NO_REPEAT[3], coords: COORDS.left_bottom },
        Vertex { position: POSITIONS_NO_REPEAT[0], normal: NORMALS_NO_REPEAT[3], coords: COORDS.right_top },
        Vertex { position: POSITIONS_NO_REPEAT[1], normal: NORMALS_NO_REPEAT[3], coords: COORDS.right_bottom },
        Vertex { position: POSITIONS_NO_REPEAT[6], normal: NORMALS_NO_REPEAT[4], coords: COORDS.left_top },
        Vertex { position: POSITIONS_NO_REPEAT[7], normal: NORMALS_NO_REPEAT[4], coords: COORDS.left_bottom },
        Vertex { position: POSITIONS_NO_REPEAT[4], normal: NORMALS_NO_REPEAT[4], coords: COORDS.right_top },
        Vertex { position: POSITIONS_NO_REPEAT[5], normal: NORMALS_NO_REPEAT[4], coords: COORDS.right_bottom },
        Vertex { position: POSITIONS_NO_REPEAT[2], normal: NORMALS_NO_REPEAT[5], coords: COORDS.left_top },
        Vertex { position: POSITIONS_NO_REPEAT[3], normal: NORMALS_NO_REPEAT[5], coords: COORDS.left_bottom },
        Vertex { position: POSITIONS_NO_REPEAT[6], normal: NORMALS_NO_REPEAT[5], coords: COORDS.right_top },
        Vertex { position: POSITIONS_NO_REPEAT[7], normal: NORMALS_NO_REPEAT[5], coords: COORDS.right_bottom },
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
        // top
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