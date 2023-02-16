use ultraviolet::Mat4;

pub struct Camera {
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
}

impl Camera {
    pub fn new(
        aspect: f32,
        fov: f32,
        near: f32, 
        far: f32
    ) -> Self {

        let view_matrix = Mat4::identity();
        let projection_matrix = ultraviolet::projection::perspective_gl(fov.to_radians(),aspect, near, far);

        Camera {
            view_matrix,
            projection_matrix
        }
    }

    pub fn update_aspect(aspect: f32) {
    }
}