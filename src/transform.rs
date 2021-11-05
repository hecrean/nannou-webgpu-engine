use nannou::prelude::{mat3, mat4, vec3, Mat3, Mat4, Quat, Vec3, Vec4};
use std::cmp::Ordering;
// http://www.opengl-tutorial.org/beginners-tutorials/tutorial-3-matrices/#translation-matrices

#[derive(Debug, Clone)]
pub enum Handedness {
    // For a coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
    Left = -1,
    // For a coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
    Right = 1,
}

pub trait Transformable {
    fn handededness(&self) -> Handedness;
    fn mat4x4(&self) -> Mat4;
    fn x_axis(&self) -> Vec3;
    fn y_axis(&self) -> Vec3;
    fn z_axis(&self) -> Vec3;
}
pub trait Orthonormal {
    fn is_orthonormal(&self) -> bool;
}

#[derive(Debug, Clone, Default)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Orthonormal for Transform {
    fn is_orthonormal(&self) -> bool {
        let id_mat4x4: Mat4 = self.mat4x4() * self.mat4x4().transpose();
        id_mat4x4.eq(&Mat4::IDENTITY)
    }
}

impl Transformable for Transform {
    fn x_axis(&self) -> Vec3 {
        let e1 = self.mat4x4().col(0);
        vec3(e1[0], e1[1], e1[2])
    }
    fn y_axis(&self) -> Vec3 {
        let e2 = self.mat4x4().col(1);
        vec3(e2[0], e2[1], e2[2])
    }
    fn z_axis(&self) -> Vec3 {
        let e3 = self.mat4x4().col(2);
        vec3(e3[0], e3[1], e3[2])
    }

    fn handededness(&self) -> Handedness {
        let handedness =
            Vec3::dot(Vec3::cross(self.x_axis(), self.y_axis()), self.y_axis()).partial_cmp(&0.0);

        match handedness {
            Some(Ordering::Less) => Handedness::Left,
            Some(Ordering::Greater) => Handedness::Right,
						Some(Ordering::Equal) => panic!("handedness can can not be 0; this implies the basis vectors are not linearly independent"),
						None => panic!("error in handedness calc")
        }
    }

    // model_matrix is an alias for the (global) transform matrix
    fn mat4x4(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
    }
}

impl Transform {
    pub fn new(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            translation,
            rotation,
            scale,
        }
    }
}
