use std::error::Error;
use glium::framebuffer::{MultiOutputFrameBuffer, SimpleFrameBuffer};
use glium::texture::{DepthTexture2d, Texture2d, UncompressedFloatFormat::F32F32F32F32, MipmapsOption::NoMipmap, DepthFormat::F32};
use glium::backend::glutin::Display;
use glium::{Surface, Program, VertexBuffer, vertex::Vertex, index::{IndicesSource, NoIndices, PrimitiveType::TrianglesList}, draw_parameters::DrawParameters, uniforms::Uniforms};
use render_object::{RenderObject, ModelMatrix, PosMatrix, LightModel};
use camera::PCamera;

pub enum RenderStage {
    PrePass,
    Lighting,
}

pub struct GBuffer<'a> {
    display: &'a Display,
    pub diffuse: Texture2d,
    pub normal: Texture2d,
    pub specular: Texture2d,
    pub depth: DepthTexture2d,
    pub light: Texture2d,
}

impl<'a> GBuffer<'a> {
    pub fn with_dimensions(display: &Display, dimensions: (u32, u32)) -> Result<GBuffer, Box<Error>> {
        let diffuse = Texture2d::empty_with_format(display, F32F32F32F32, NoMipmap, dimensions.0, dimensions.1)?;
        let normal = Texture2d::empty_with_format(display, F32F32F32F32, NoMipmap, dimensions.0, dimensions.1)?;
        let specular = Texture2d::empty_with_format(display, F32F32F32F32, NoMipmap, dimensions.0, dimensions.1)?;
        let depth = DepthTexture2d::empty_with_format(display, F32, NoMipmap, dimensions.0, dimensions.1)?;
        let light = Texture2d::empty_with_format(display, F32F32F32F32, NoMipmap, dimensions.0, dimensions.1)?;


        Ok(GBuffer {
            display,
            diffuse,
            normal,
            specular, 
            depth,
            light
        })
    }

    pub fn resize(&mut self, dimensions: (u32, u32)) -> Result<(), Box<Error>> {
        *self = GBuffer::with_dimensions(self.display, dimensions)?; 
        Ok(())
    }

    pub fn framebuffer(&self) -> Result<MultiOutputFrameBuffer, Box<Error>> {
        let output = &[("diffuse", &self.diffuse), ("normal", &self.normal), ("specular", &self.specular)];
        let framebuffer = MultiOutputFrameBuffer::with_depth_buffer(*&self.display, output.iter().cloned(), &self.depth)?;
        Ok(framebuffer)
    }

    pub fn lightbuffer(&self) -> Result<SimpleFrameBuffer, Box<Error>> {
        let lightbuffer = SimpleFrameBuffer::with_depth_buffer(*&self.display, &self.light, &self.depth)?;
        Ok(lightbuffer)

    }
}

pub struct FrameBuffers<'a> {
    pub framebuffer: MultiOutputFrameBuffer<'a>,
    pub lightbuffer: SimpleFrameBuffer<'a>,
    stage: RenderStage,
}

impl<'a> FrameBuffers<'a> {
    pub fn new(gbuffer: &'a GBuffer<'a>) -> Result<FrameBuffers<'a>, Box<Error>> {
        Ok(
            FrameBuffers {
                framebuffer: gbuffer.framebuffer()?,
                lightbuffer: gbuffer.lightbuffer()?,
                stage: RenderStage::PrePass,
        })
    }

    pub fn reset(&mut self){
        self.framebuffer.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
        self.lightbuffer.clear_color(0.0, 0.0, 0.0, 0.0);
        self.stage = RenderStage::PrePass;
    }

    pub fn draw<'b, V, I, U>(&mut self, buffer: &VertexBuffer<V>, indices: I, program: &Program, uniforms: &U, draw_parameters: &DrawParameters) -> Result<(), Box<Error>>
    where
        V: Vertex, 
        I: Into<IndicesSource<'b>>, 
        U: Uniforms
    {
        match self.stage {
            RenderStage::PrePass => {
                self.framebuffer.draw(buffer, indices, program, uniforms, draw_parameters)?;
            }
            RenderStage::Lighting => {
                self.lightbuffer.draw(buffer, indices, program, uniforms, draw_parameters)?;
            }   
        }
        Ok(())
    }

    pub fn draw_object<T: ModelMatrix>(&mut self, render_object: &RenderObject<T>, camera: &PCamera, program: &Program, draw_parameters: &DrawParameters) -> Result<(), Box<Error>> {
        let uniforms = uniform! {
            view: *camera.view_matrix().as_ref(),
            model: render_object.model_matrix.matrix(),
            normal_map: render_object.normal_tex,
            depth_map: render_object.depth_tex,
            diffuse_map: render_object.diffuse_tex,
            specular_map: render_object.specular_tex,
            depth_scale: render_object.depth_scale,
            eye: *camera.position.coords.as_ref(),
        };

        self.draw(render_object.buffer, NoIndices(TrianglesList), program, &uniforms, draw_parameters)?;
        Ok(())
    }

    pub fn draw_light<T: PosMatrix>(&mut self, shininess: f32, light: &LightModel<T>, camera: &PCamera, program: &Program, draw_parameters: &DrawParameters, gbuffer: &GBuffer) -> Result<(), Box<Error>> {
        let perspective_mat = camera.projection_matrix();
        let look_at_mat = camera.look_at_matrix();
        let uniforms = uniform! {
            view: *(perspective_mat * look_at_mat).as_ref(),
            model: light.position.matrix(),
            normal_tex: &gbuffer.normal,
            depth_tex: &gbuffer.depth,
            diffuse_tex: &gbuffer.diffuse,
            specular_tex: &gbuffer.specular,
            eye: *camera.position.coords.as_ref(),
            inv_projection: *camera.inv_view_matrix().as_ref(),
            shininess: shininess,
            T1: perspective_mat[(2, 2)],
            T2: perspective_mat[(2, 3)],
            light_pos: light.position.position(),
            light_colour: light.colour,
        };

        self.draw(&light.buffer, NoIndices(TrianglesList), program, &uniforms, draw_parameters)?;
        Ok(())
    }

    pub fn next_stage(&mut self) {
        self.stage = RenderStage::PrePass;
    }
}