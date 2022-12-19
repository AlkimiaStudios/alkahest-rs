use ultraviolet::{Vec3, Mat4};

pub struct Transform {
    position: Vec3,
    rotation: Vec3,
    scale: Vec3,
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            position: Vec3::zero(),
            rotation: Vec3::zero(),
            scale: Vec3::one(),
        }
    }

    pub fn get_matrix(&self) -> Mat4 {
        // Translation is easy
        let translation: Mat4 = Mat4::identity().translated(&self.position);
        
        // Rotation is built from euler angles, assuming each item in
        // self.rotation is set in radians
        let rotation: Mat4 = Mat4::from_euler_angles(self.rotation.z, self.rotation.x, self.rotation.y);

        // Using non-uniform scale to allow scaling on different axes
        let scale: Mat4 = Mat4::from_nonuniform_scale(self.scale);

        // The order here is important! Matrix multiplication is NOT commutative
        return translation * rotation * scale;
    }

    pub fn get_position(&self) -> &Vec3 {
        &self.position
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    pub fn get_rotation(&self) -> &Vec3 {
        &self.rotation
    }

    pub fn set_rotation(&mut self, rotation: Vec3) {
        self.rotation = rotation;
    }

    pub fn get_scale(&self) -> &Vec3 {
        &self.scale
    }

    pub fn set_scale(&mut self, scale: Vec3) {
        self.scale = scale;
    }
}
