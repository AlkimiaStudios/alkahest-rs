use ultraviolet::{Mat4,Vec3,projection::{orthographic_gl,perspective_gl}};

pub trait Camera {
    fn recalculate_matrices(&mut self);

    fn get_position(&self) -> &Vec3;
    fn set_position(&mut self, position: Vec3);
    fn get_rotation(&self) -> &Vec3;
    fn set_rotation(&mut self, rotation: Vec3);

    fn get_view_matrix(&self) -> &Mat4;
    fn get_projection_view_matrix(&self) -> &Mat4;
}

pub struct Camera2D {
    projection_matrix: Mat4,
    view_matrix: Mat4,
    projection_view_matrix: Mat4,

    position: Vec3,
    rotation: Vec3,
}

pub struct Camera3D {
    projection_matrix: Mat4,
    view_matrix: Mat4,
    projection_view_matrix: Mat4,

    position: Vec3,
    rotation: Vec3,
}

impl Camera2D {
    pub fn new(left: f32, right: f32, bottom: f32, top: f32) -> Self {
        let pm = orthographic_gl(left, right, bottom, top, -1., 1.);
        let vm = Mat4::identity();

        Camera2D {
            projection_matrix: pm,
            view_matrix: vm,
            projection_view_matrix: pm * vm,
            position: Vec3::zero(),
            rotation: Vec3::zero(),
        }
    }
}

impl Camera3D {
    pub fn new(fov: f32, aspect_ratio: f32, near_plane: f32, far_plane: f32) -> Self {
        let pm = perspective_gl(fov, aspect_ratio, near_plane, far_plane);
        let vm = Mat4::identity();

        Camera3D {
            projection_matrix: pm,
            view_matrix: vm,
            projection_view_matrix: pm * vm,
            position: Vec3::zero(),
            rotation: Vec3::zero(),
        }
    }
}

impl Camera for Camera2D {
    fn recalculate_matrices(&mut self) {
        let transform: Mat4 = Mat4::identity().translated(&self.position);
        let transform = transform * Mat4::from_rotation_z(self.rotation.z); 

        // self.view_matrix = transform.inversed();
        self.view_matrix = transform;
        self.projection_view_matrix = self.projection_matrix * self.view_matrix;
    }

    fn get_position(&self) -> &Vec3 {
        &self.position
    }

    fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    fn get_rotation(&self) -> &Vec3 {
        &self.rotation
    }

    fn set_rotation(&mut self, rotation: Vec3) {
        self.rotation = rotation;
    }

    fn get_view_matrix(&self) -> &Mat4 {
        &self.view_matrix
    }

    fn get_projection_view_matrix(&self) -> &Mat4 {
        &self.projection_view_matrix
    }
}

impl Camera for Camera3D {
    fn recalculate_matrices(&mut self) {
        let transform: Mat4 = Mat4::identity().translated(&self.position);
        let transform = transform * Mat4::from_euler_angles(self.rotation.z, self.rotation.x, self.rotation.y); 

        self.view_matrix = transform.inversed();
        self.projection_view_matrix = self.projection_matrix * self.view_matrix;
    }

    fn get_position(&self) -> &Vec3 {
        &self.position
    }

    fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    fn get_rotation(&self) -> &Vec3 {
        &self.rotation
    }

    fn set_rotation(&mut self, rotation: Vec3) {
        self.rotation = rotation;
    }

    fn get_view_matrix(&self) -> &Mat4 {
        &self.view_matrix
    }

    fn get_projection_view_matrix(&self) -> &Mat4 {
        &self.projection_view_matrix
    }
}
