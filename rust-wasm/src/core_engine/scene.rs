use crate::objects::Mesh;

/// Scene manages a collection of meshes (similar to Kansei's Scene)
pub struct Scene {
    pub children: Vec<Mesh>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    /// Add a mesh to the scene
    pub fn add(&mut self, mesh: Mesh) {
        self.children.push(mesh);
    }

    /// Remove a mesh from the scene by index
    pub fn remove(&mut self, index: usize) -> Option<Mesh> {
        if index < self.children.len() {
            Some(self.children.remove(index))
        } else {
            None
        }
    }

    /// Clear all meshes from the scene
    pub fn clear(&mut self) {
        self.children.clear();
    }

    /// Get number of children
    pub fn len(&self) -> usize {
        self.children.len()
    }

    /// Check if scene is empty
    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

