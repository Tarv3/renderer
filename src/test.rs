use math::{Triangle, PointNormal};
use Vertex;
use {Vec3, Vec2};

pub fn gen_plane() -> Vec<Vertex> {
    let mut vertices = Vec::with_capacity(6);

    let t1 = Triangle {
        p1: PointNormal {  
                position: Vec3::new(-1.0, 1.0, 0.0),
                normal: Vec3::new(0.0, 0.0, 1.0),
                tex_coord: Vec2::new(0.0, 1.0)
            },
        p2: PointNormal {  
                position: Vec3::new(-1.0, -1.0, 0.0),
                normal: Vec3::new(0.0, 0.0, 1.0),
                tex_coord: Vec2::new(0.0, 0.0)
            },
        p3: PointNormal {  
                position: Vec3::new(1.0, -1.0, 0.0),
                normal: Vec3::new(0.0, 0.0, 1.0),
                tex_coord: Vec2::new(1.0, 0.0)
            }
    };

    let t2 = Triangle {
        p1: PointNormal {  
                position: Vec3::new(-1.0, 1.0, 0.0),
                normal: Vec3::new(0.0, 0.0, 1.0),
                tex_coord: Vec2::new(0.0, 1.0)
            },
        p2: PointNormal {  
                position: Vec3::new(1.0, -1.0, 0.0),
                normal: Vec3::new(0.0, 0.0, 1.0),
                tex_coord: Vec2::new(1.0, 0.0)
            },
        p3: PointNormal {  
                position: Vec3::new(1.0, 1.0, 0.0),
                normal: Vec3::new(0.0, 0.0, 1.0),
                tex_coord: Vec2::new(1.0, 1.0)
            }
    };

    vertices.extend_from_slice(&t1.into_vertices());
    vertices.extend_from_slice(&t2.into_vertices());

    return vertices;
}