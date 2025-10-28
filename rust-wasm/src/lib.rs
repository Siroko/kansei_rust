use wasm_bindgen::prelude::*;

mod core_engine;
mod geometries;
mod math;
mod objects;

pub use core_engine::{Camera, CameraControls, Renderer, Scene};
pub use geometries::{BoxGeometry, Geometry, PlaneGeometry, Vertex};
pub use math::{Matrix4, Vector3};
pub use objects::Mesh;

/// Main Engine class that ties everything together (inspired by Kansei)
#[wasm_bindgen]
pub struct Engine {
    renderer: Renderer,
    scene: Scene,
    camera_controls: CameraControls,
    time: f32,
}

#[wasm_bindgen]
impl Engine {
    pub async fn new(canvas_id: &str, width: u32, height: u32) -> Result<Engine, JsValue> {
        log::info!("Creating new Engine...");

        let renderer = Renderer::new(canvas_id, false).await?;
        let scene = Scene::new();
        let aspect = width as f32 / height as f32;
        let camera = Camera::new(75.0, 0.1, 1000.0, aspect);
        
        // Create camera controls with target at origin and radius of 50
        let target = Vector3::new(0.0, 0.0, 0.0);
        let camera_controls = CameraControls::new(camera, target, 50.0, canvas_id)?;

        let mut engine = Engine {
            renderer,
            scene,
            camera_controls,
            time: 0.0,
        };
        
        // Initialize default scene
        engine.init_scene();
        
        Ok(engine)
    }

    /// Update engine state (animations, physics, etc.)
    /// Call this every frame before render
    /// delta_time: time multiplier (1.0 = 60fps baseline)
    pub fn update(&mut self, delta_time: f32) {
        // Update camera controls
        self.camera_controls.update(delta_time);
        
        // Animate all meshes in the grid with wave effect
        let grid_size = 10;
        
        for (i, mesh) in self.scene.children.iter_mut().enumerate() {
            // Calculate grid position
            let x_idx = (i % grid_size * 2) as f32;
            let y_idx = (i / grid_size) as f32;
            
            // Create wave effect based on position and time
            let wave = ((x_idx + y_idx) * 0.05 + self.time * 2.0).sin();
            
            // Animate Z position with wave
            mesh.position.z = wave * 15.0;
            
            // Rotate based on position
            mesh.rotation.y += 0.02 * delta_time;
            mesh.rotation.x = wave * 0.3;
        }
    }

    /// Render the scene
    pub fn render(&mut self) -> Result<(), JsValue> {
        self.time += 0.016;
        self.renderer.render(&mut self.scene, self.camera_controls.camera())
    }

    /// Resize the renderer
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.renderer.set_size(width, height);
        self.camera_controls.camera_mut().update_aspect(width as f32 / height as f32);
        self.camera_controls.set_window_size(width as f32, height as f32);
    }
}

// Private Rust-only methods (not exposed to JavaScript)
impl Engine {
    /// Initialize the default scene with demo objects
    fn init_scene(&mut self) {
        log::info!("Initializing default scene...");
        
        // Set clear color
        self.renderer.set_clear_color(0.0, 0.0, 0.0, 1.0);
        
        // Create a 10x10 grid of cubes on the XY plane
        let grid_size = 10;
        let spacing = 1.0;
        let cube_size = 1.0;
        
        for i in 0..grid_size {
            for j in 0..grid_size {
                let x = (i as f32 - grid_size as f32 / 2.0) * spacing * 2.0;
                let y = (j as f32 - grid_size as f32 / 2.0) * spacing;
                let z = 0.0;
                
                let geometry = BoxGeometry::new(cube_size, cube_size, cube_size);
                let mut mesh = Mesh::new(geometry);
                mesh.position = Vector3::new(x, y, z);
                self.scene.add(mesh);
            }
        }
        
        log::info!("Scene initialized with {} meshes", self.scene.len());
    }

    /// Get number of meshes in scene
    pub fn mesh_count(&self) -> usize {
        self.scene.len()
    }

    /// Set renderer clear color
    pub fn set_clear_color(&mut self, r: f64, g: f64, b: f64, a: f64) {
        self.renderer.set_clear_color(r, g, b, a);
    }

    /// Get current time
    pub fn get_time(&self) -> f32 {
        self.time
    }

    /// Clear all meshes from the scene
    pub fn clear_scene(&mut self) {
        self.scene.clear();
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}
