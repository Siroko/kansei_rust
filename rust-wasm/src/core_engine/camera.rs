use crate::math::Vector3;
use glam::{Mat4, Vec3};

/// Camera with perspective projection (similar to Kansei's Camera)
#[derive(Debug)]
pub struct Camera {
    pub position: Vector3,
    pub rotation: Vector3,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
    // Store the look-at target for view matrix calculation
    look_at_target: Option<Vector3>,
}

impl Camera {
    /// Create a new camera with perspective projection
    /// 
    /// # Arguments
    /// * `fov` - Field of view in degrees
    /// * `near` - Near clipping plane
    /// * `far` - Far clipping plane
    /// * `aspect` - Aspect ratio (width / height)
    pub fn new(fov: f32, near: f32, far: f32, aspect: f32) -> Self {
        Self {
            position: Vector3::new(0.0, 0.0, 5.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            fov: fov.to_radians(),
            aspect,
            near,
            far,
            look_at_target: None,
        }
    }

    /// Make the camera look at a specific target point
    pub fn look_at(&mut self, target: &Vector3) {
        self.look_at_target = Some(*target);
    }

    /// Get the projection matrix using glam
    pub fn projection_matrix_glam(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect, self.near, self.far)
    }

    /// Get the view matrix using glam
    pub fn view_matrix_glam(&self) -> Mat4 {
        let eye = Vec3::new(self.position.x, self.position.y, self.position.z);
        
        // Use look_at_target if set, otherwise look in the -Z direction
        let center = if let Some(target) = self.look_at_target {
            Vec3::new(target.x, target.y, target.z)
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        };
        
        let up = Vec3::new(0.0, 1.0, 0.0);
        Mat4::look_at_rh(eye, center, up)
    }

    /// Update aspect ratio (call this on window resize)
    pub fn update_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
    }

    /// Set field of view (in degrees)
    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov.to_radians();
    }
}

