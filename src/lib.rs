extern crate nalgebra as na;
#[macro_use]
extern crate glium;

pub mod math;
pub mod test;
pub mod camera;
pub mod gbuffer;
pub mod render_object;

pub type Vec3 = na::Vector3<f32>;
pub type Vec2 = na::Vector2<f32>;
pub type Mat4 = na::Matrix4<f32>;
pub type Pnt3 = na::Point3<f32>;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tangent: [f32; 3],
    pub bitangent: [f32; 3],
    pub tex_coord: [f32; 2],
}

implement_vertex!(Vertex, position, normal, tangent, bitangent, tex_coord);

#[derive(Copy, Clone, Debug)]
pub struct SimpleVertex {
    pub position: [f32; 3],
}

implement_vertex!(SimpleVertex, position);
