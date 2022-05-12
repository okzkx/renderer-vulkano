pub(crate) mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "src/shaders/vert.glsl",
        types_meta: {
            use bytemuck::{Pod, Zeroable};

            #[derive(Clone, Copy, Zeroable, Pod)]
        },
    }
}

pub(crate) mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "src/shaders/frag.glsl"
    }
}