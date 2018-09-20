use na::{dot, normalize};
use {Vec2, Vec3, Vertex};

pub struct PointNormal {
    pub position: Vec3,
    pub normal: Vec3,
    pub tex_coord: Vec2,
}

pub struct Triangle {
    pub p1: PointNormal,
    pub p2: PointNormal,
    pub p3: PointNormal,
}

impl Triangle {
    pub fn into_vertices(&self) -> [Vertex; 3] {
        let v1 = self.p2.position - self.p1.position;
        let t1 = self.p2.tex_coord - self.p1.tex_coord;
        
        let v2 = self.p3.position - self.p1.position;
        let t2 = self.p3.tex_coord - self.p1.tex_coord;

        let (tangent, bitangent) = tangent_space_from(v1, t1, v2, t2);
        let p1_basis = OrthoBasis::from_basis(self.p1.normal, tangent, bitangent);
        let p2_basis = OrthoBasis::from_basis(self.p2.normal, tangent, bitangent);
        let p3_basis = OrthoBasis::from_basis(self.p3.normal, tangent, bitangent);

        [
            Vertex {
                position: *self.p1.position.as_ref(),
                normal: *self.p1.normal.as_ref(),
                tangent: *p1_basis.v2.as_ref(),
                bitangent: *p1_basis.v3.as_ref(),
                tex_coord: *self.p1.tex_coord.as_ref(),
            },
            Vertex {
                position: *self.p2.position.as_ref(),
                normal: *self.p2.normal.as_ref(),
                tangent: *p2_basis.v2.as_ref(),
                bitangent: *p2_basis.v3.as_ref(),
                tex_coord: *self.p2.tex_coord.as_ref(),
            },
            Vertex {
                position: *self.p3.position.as_ref(),
                normal: *self.p3.normal.as_ref(),
                tangent: *p3_basis.v2.as_ref(),
                bitangent: *p3_basis.v3.as_ref(),
                tex_coord: *self.p3.tex_coord.as_ref(),
            }
        ]
    }
}

pub struct OrthoBasis {
    pub v1: Vec3,
    pub v2: Vec3,
    pub v3: Vec3,
}

impl OrthoBasis {
    // Creates an ortho basis from the basis {v1, v2, v3} using gram schmidt
    pub fn from_basis(v1: Vec3, v2: Vec3, v3: Vec3) -> OrthoBasis {
        let new_v2 = normalize(&(v2 - dot(&v2, &v1) * v1));
        let new_v3 = normalize(&(v3 - dot(&v3, &v1) * v1 - dot(&v3, &new_v2) * new_v2));

        return OrthoBasis {
            v1,
            v2: new_v2,
            v3: new_v3,
        };
    }
}

pub fn tangent_space_from(v1: Vec3, t1: Vec2, v2: Vec3, t2: Vec2) -> (Vec3, Vec3) {
    let det_a = t1.x * t2.y - t1.y * t2.x;
    assert!(det_a != 0.0);
    let inv_det_a = 1.0 / det_a;

    let tangent = inv_det_a * Vec3::new(
        v1.x * t2.y - v2.x * t1.y,
        v1.y * t2.y - v2.y * t1.y,
        v1.z * t2.y - v2.z * t1.y,
    );

    let bitangent = inv_det_a * Vec3::new(
        -v1.x * t2.x + v2.x * t1.x,
        -v1.y * t2.x + v2.y * t1.x,
        -v1.z * t2.x + v2.z * t1.x,
    );

    (tangent, bitangent)
}

// Will return the change in angle that is allowed by min and max
pub fn clamp_rotation(current_angle: f32, delta_angle: f32, min: f32, max: f32) -> f32 {
    let new_angle = current_angle + delta_angle;
    if new_angle > max {
        max - current_angle
    }  
    else if new_angle < min {
        min - current_angle
    }
    else {
        delta_angle
    }
}

pub fn project(a: &Vec3, b: &Vec3) -> Vec3 {
    dot(a, b) / dot(b, b) * b
}

pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    assert!(min < max);
    
    if value > max || value.is_nan()|| value.is_infinite() {
        max
    }
    else if value < min {
        min
    }
    else {
        value
    }
}