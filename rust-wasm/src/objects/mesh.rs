use crate::geometries::Geometry;
use crate::math::{Matrix4, Vector3};
use wgpu::util::DeviceExt;
use glam::{Mat4, Vec3 as GlamVec3, Quat};

/// Mesh combines geometry with transformation properties (similar to Kansei's Mesh)
pub struct Mesh {
    pub position: Vector3,
    pub rotation: Vector3,
    pub scale: Vector3,
    pub visible: bool,
    pub geometry: Geometry,
    pub vertex_buffer: Option<wgpu::Buffer>,
    pub index_buffer: Option<wgpu::Buffer>,
    pub uniform_buffer: Option<wgpu::Buffer>,
    pub bind_group: Option<wgpu::BindGroup>,
}

impl Mesh {
    /// Create a new mesh from geometry
    pub fn new(geometry: Geometry) -> Self {
        Self {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
            visible: true,
            geometry,
            vertex_buffer: None,
            index_buffer: None,
            uniform_buffer: None,
            bind_group: None,
        }
    }

    /// Create GPU buffers for this mesh
    pub(crate) fn create_buffers(&mut self, device: &wgpu::Device) {
        self.vertex_buffer = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&self.geometry.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        }));

        self.index_buffer = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&self.geometry.indices),
            usage: wgpu::BufferUsages::INDEX,
        }));
    }

    /// Calculate the model matrix from position, rotation, and scale
    pub fn model_matrix(&self) -> Matrix4 {
        let translation = Matrix4::translation(self.position.x, self.position.y, self.position.z);
        let rotation_x = Matrix4::rotation_x(self.rotation.x);
        let rotation_y = Matrix4::rotation_y(self.rotation.y);
        let rotation_z = Matrix4::rotation_z(self.rotation.z);
        let scale = Matrix4::scale(self.scale.x, self.scale.y, self.scale.z);

        translation
            .multiply(&rotation_y)
            .multiply(&rotation_x)
            .multiply(&rotation_z)
            .multiply(&scale)
    }

    /// Set visibility
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Toggle visibility
    pub fn toggle_visible(&mut self) {
        self.visible = !self.visible;
    }
    
    /// Calculate model matrix using glam (proven math library)
    pub fn model_matrix_glam(&self) -> Mat4 {
        let translation = GlamVec3::new(self.position.x, self.position.y, self.position.z);
        let rotation = Quat::from_euler(glam::EulerRot::XYZ, self.rotation.x, self.rotation.y, self.rotation.z);
        let scale = GlamVec3::new(self.scale.x, self.scale.y, self.scale.z);
        
        Mat4::from_scale_rotation_translation(scale, rotation, translation)
    }
}

