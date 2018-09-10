use glium::texture::Texture2d;
use glium::VertexBuffer;
use Vertex;
use SimpleVertex;

pub trait ModelMatrix {
    fn matrix(&self) -> [[f32; 4]; 4];
}

pub trait PosMatrix : ModelMatrix {
    fn position(&self) -> [f32; 3];
}

pub struct RenderObject<'a, T: ModelMatrix> {
    pub model_matrix: T,
    pub buffer: &'a VertexBuffer<Vertex>,
    pub diffuse_tex: &'a Texture2d,
    pub specular_tex: &'a Texture2d,
    pub normal_tex: &'a Texture2d,
    pub depth_tex: &'a Texture2d,
    pub depth_scale: f32,
}

impl<'a, T: ModelMatrix> RenderObject<'a, T> {
    pub fn new(
        model_matrix: T,
        buffer: &'a VertexBuffer<Vertex>,
        diffuse_tex: &'a Texture2d,
        specular_tex: &'a Texture2d,
        normal_tex: &'a Texture2d,
        depth_tex: &'a Texture2d,
        depth_scale: f32,
    ) -> RenderObject<'a, T> {
        RenderObject {
            model_matrix,
            buffer,
            diffuse_tex,
            specular_tex,
            normal_tex,
            depth_tex, 
            depth_scale
        }
    }
}

pub struct LightModel<'a, T: PosMatrix>  {
    pub position: T,
    pub colour: [f32; 3],
    pub buffer: &'a VertexBuffer<SimpleVertex>
}

impl<'a, T: PosMatrix> LightModel<'a, T> {
    pub fn new(position: T, colour: [f32; 3], buffer: &'a VertexBuffer<SimpleVertex>) -> LightModel<'a, T> {
        LightModel {
            position,
            colour, 
            buffer
        }
    }
}