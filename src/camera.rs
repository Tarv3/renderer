use math::{clamp, clamp_rotation, project};
use na::{
    angle, base::Matrix, geometry::IsometryMatrix3, norm, Orthographic3, Perspective3, Rotation3,
    Unit,
};
use std::f32::consts::{FRAC_1_PI, PI};
use {Mat4, Pnt3, Vec3};

#[derive(Clone, Copy, Debug)]
pub enum Projection {
    Orthographic(Orthographic3<f32>),
    Perspective(Perspective3<f32>),
}

impl Projection {
    pub fn persp(perspective: Perspective3<f32>) -> Projection {
        Projection::Perspective(perspective)
    }

    pub fn ortho(orthographic: Orthographic3<f32>) -> Projection {
        Projection::Orthographic(orthographic)
    }
    
    pub fn znear(&self) -> f32 {
        match &self {
            Projection::Perspective(persp) => persp.znear(),
            Projection::Orthographic(ortho) => ortho.znear(),
        }
    }

    pub fn zfar(&self) -> f32 {
        match &self {
            Projection::Perspective(persp) => persp.zfar(),
            Projection::Orthographic(ortho) => ortho.zfar(),
        }
    }

    pub fn as_matrix(&self) -> Mat4 {
        match &self {
            Projection::Perspective(persp) => *persp.as_matrix(),
            Projection::Orthographic(ortho) => *ortho.as_matrix(),
        }
    }

    pub fn inverse_as_matrix(&self) -> Mat4 {
        match &self {
            Projection::Perspective(persp) => persp.inverse(),
            Projection::Orthographic(ortho) => ortho.inverse(),
        }
    }

    pub fn aspect_ratio(&self) -> f32 {
        match &self {
            Projection::Perspective(persp) => persp.aspect(),
            Projection::Orthographic(ortho) => {
                let width = ortho.right() - ortho.left();
                let height = ortho.top() - ortho.bottom();
                width / height
            },
        }
    }

    pub fn set_aspect(&mut self, aspect: f32) {
        match self {
            Projection::Perspective(ref mut persp) => persp.set_aspect(aspect),
            Projection::Orthographic(ref mut ortho) => {
                let centre = (ortho.right() + ortho.left()) * 0.5;
                let vertical = ortho.top() - ortho.bottom();
                let horiz_rad = (vertical * aspect) * 0.5;

                ortho.set_right(centre + horiz_rad);
                ortho.set_left(centre - horiz_rad);
            }
        }
    }

    pub fn to_perspective(&self) -> Projection {
        match self {
            Projection::Orthographic(ortho) => {
                let zfar = ortho.zfar();
                let width = ortho.right() - ortho.left();
                let fov = clamp(width / zfar, -1.0, 1.0).atan();
                let aspect = (ortho.right() - ortho.left()) / (ortho.top() - ortho.bottom());

                Projection::Perspective(Perspective3::new(
                    aspect,
                    4.0 * fov,
                    ortho.znear(),
                    ortho.zfar(),
                ))
            }
            Projection::Perspective(persp) => Projection::persp(*persp),
        }
    }

    pub fn to_orthographic(&self) -> Projection {
        match self {
            Projection::Perspective(persp) => Projection::Orthographic(Orthographic3::from_fov(
                persp.aspect(),
                persp.fovy() * 0.5,
                persp.znear(),
                persp.zfar(),
            )),
            Projection::Orthographic(ortho) => Projection::ortho(*ortho),
        }
    }

    pub fn zoomed_matrix(&self, scale: f32) -> Mat4 {
        match self {
            Projection::Perspective(mut persp) => {
                let fovy = persp.fovy();
                let y = fovy.tan();
                let mut new_fovy = (scale * y).atan();
                // new_fovy = clamp(new_fovy, 0.0001, 1.5699);
                persp.set_fovy(new_fovy);

                *persp.as_matrix()
            }
            Projection::Orthographic(mut ortho) => {
                let scale = scale * 0.5;
                let left = ortho.left();
                let right = ortho.right();
                let top = ortho.top();
                let bottom = ortho.bottom();
                ortho.set_left(left * scale);
                ortho.set_right(right * scale);
                ortho.set_top(top * scale);
                ortho.set_bottom(bottom * scale);

                *ortho.as_matrix()
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PCamera {
    pub vertical_angle: f32,
    pub position: Pnt3,
    pub look_at: Pnt3,
    pub up: Unit<Vec3>,
    pub projection: Projection,
}

impl PCamera {
    pub fn new(position: Vec3, look_at: Vec3, up: Vec3, projection: Projection) -> PCamera {
        let vertical_angle = angle(&(look_at - position), &up);
        assert!(vertical_angle > 0.0 && vertical_angle < PI);

        let position = Pnt3::from_coordinates(position);
        let look_at = Pnt3::from_coordinates(look_at);
        let up = Unit::new_normalize(up);

        PCamera {
            vertical_angle,
            position,
            look_at,
            up,
            projection,
        }
    }
    
    pub fn znear(&self) -> f32 {
        self.projection.znear()
    }
    
    pub fn zfar(&self) -> f32 {
        self.projection.zfar()
    }

    pub fn perspective_projection(&mut self) {
        self.projection = self.projection.to_perspective();
    }

    pub fn orthographic_projection(&mut self) {
        self.projection = self.projection.to_orthographic();
    }

    pub fn new_preset_perspective(position: Vec3, look_at: Vec3, up: Vec3) -> PCamera {
        let perspective = Perspective3::new(1.0, PI * 0.5, 0.1, 100.0);
        PCamera::new(position, look_at, up, Projection::persp(perspective))
    }

    pub fn set_aspect(&mut self, aspect: f32) {
        self.projection.set_aspect(aspect);
    }

    pub fn set_aspect_from_dims(&mut self, dimensions: (f64, f64)) {
        let aspect = dimensions.0 / dimensions.1;
        self.projection.set_aspect(aspect as f32);
    }

    pub fn forward_vec(&self) -> Unit<Vec3> {
        Unit::new_normalize(self.look_at - self.position)
    }

    pub fn forward_ortho_vec(&self) -> Unit<Vec3> {
        let look_to = self.look_at - self.position;
        Unit::new_normalize(look_to - project(&look_to, &self.up))
    }

    pub fn right_vec(&self) -> Unit<Vec3> {
        Unit::new_normalize(Matrix::cross(&self.forward_vec(), &self.up))
    }

    // A positive angle will rotate it "down". Will force it to be -3.13 <= angle <= 3.13
    pub fn rotate_view_vertical(&mut self, angle: f32) {
        let angle = clamp_rotation(self.vertical_angle, -angle, 0.1, 3.1);
        let axis = self.right_vec();
        let rotation = Rotation3::from_axis_angle(&axis, angle);
        let to_look_at = self.look_at - self.position;

        self.vertical_angle += angle;
        self.look_at = Pnt3::from_coordinates(self.position.coords + rotation * to_look_at);
    }

    // A positive angle will rotate it "left"
    pub fn rotate_view_horizontal(&mut self, angle: f32) {
        let to_look_at = self.look_at - self.position;
        let rotation = Rotation3::from_axis_angle(&self.up, angle);

        self.look_at = Pnt3::from_coordinates(self.position.coords + rotation * to_look_at);
    }

    // Positive angle will rotate counter clockwise
    pub fn rotate_around_look_horizontal(&mut self, angle: f32) {
        let to_pos = self.position - self.look_at;
        let rotation = Rotation3::from_axis_angle(&self.up, angle);

        self.position = self.look_at + rotation * to_pos;
    }

    pub fn rotate_around_look_vertical(&mut self, angle: f32) {
        let to_pos = self.position - self.look_at;
        let angle = clamp_rotation(self.vertical_angle, angle, 0.05, 3.10);
        let rotation = Rotation3::from_axis_angle(&self.right_vec(), -angle);

        self.vertical_angle += angle;
        self.position = self.look_at + rotation * to_pos;
    }

    // Positive distance will rotate counter clockwise
    pub fn pos_around_look_with_dis(&mut self, distance: f32) {
        let to_look = self.look_at - self.position;
        let radius = norm(&(to_look - project(&to_look, &self.up)));
        let circumference = 2.0 * PI * radius;
        let angle = distance * circumference * 0.5 * FRAC_1_PI;

        self.rotate_around_look_horizontal(angle);
    }

    pub fn move_unlocked(&mut self, value: &Vec3) {
        self.position += value;
        self.look_at += value;
    }

    // Doesnt move the look at with the position
    pub fn move_locked(&mut self, value: &Vec3) {
        self.position += value;
    }

    pub fn move_forward(&mut self, distance: f32) {
        let forward = self.forward_vec();

        self.move_unlocked(&(forward.unwrap() * distance));
    }

    // Positive distance will move to the right
    pub fn move_sideways(&mut self, distance: f32) {
        let sideways = self.right_vec();

        self.move_unlocked(&(sideways.unwrap() * distance));
    }

    pub fn move_up(&mut self, distance: f32) {
        let movement = *self.up.as_ref() * distance;

        self.move_unlocked(&movement);
    }
    
    pub fn move_up_relative(&mut self, distance: f32) {
        let up = *self.up.as_ref();
        let facing = self.look_at - self.position;
        let dir = ::na::normalize(&(up - project(&up, &facing)));
        let movement = dir * distance;

        self.move_unlocked(&movement);
    }

    pub fn move_forward_ortho_up(&mut self, distance: f32) {
        let forward = self.forward_ortho_vec();
        self.move_unlocked(&(*forward.as_ref() * distance));
    }

    pub fn set_relative_look(&mut self, new_relative: &Vec3) {
        self.look_at = self.position + new_relative;
    }

    pub fn projection_matrix(&self) -> Mat4 {
        self.projection.as_matrix()
    }

    pub fn look_at_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(&self.position, &self.look_at, &self.up)
    }

    pub fn view_matrix(&self) -> Mat4 {
        self.projection_matrix() * self.look_at_matrix()
    }

    pub fn zoomed_view_matrix(&self, scale: f32) -> Mat4 {
        self.projection.zoomed_matrix(scale) * self.look_at_matrix()
    }

    pub fn inv_view_matrix(&self) -> Mat4 {
        let look_at_mat = IsometryMatrix3::look_at_rh(&self.position, &self.look_at, &self.up);
        look_at_mat.inverse().to_homogeneous() * self.projection.inverse_as_matrix()
    }
}
