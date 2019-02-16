use nalgebra_glm::*;

pub struct Camera {
    projection_: Mat4x4,
}

impl Camera {
    pub fn new() -> Self {
        let persp : Mat4x4 = perspective_fov(90.0 ,320.0, 240.0, 0.01, 100.0);
        Camera{ projection_: persp }
    }

    pub fn projection(&self) -> &Mat4x4 {
        &self.projection_
    }

}