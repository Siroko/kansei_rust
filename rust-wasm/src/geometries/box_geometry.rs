use super::{Geometry, Vertex};

/// Box geometry (similar to Kansei's BoxGeometry)
pub struct BoxGeometry;

impl BoxGeometry {
    pub fn new(width: f32, height: f32, depth: f32) -> Geometry {
        let hw = width / 2.0;
        let hh = height / 2.0;
        let hd = depth / 2.0;

        let vertices = vec![
            // Front face (z+)
            Vertex { position: [-hw, -hh, hd], normal: [0.0, 0.0, 1.0], uv: [0.0, 1.0], color: [1.0, 0.0, 0.0] },
            Vertex { position: [hw, -hh, hd], normal: [0.0, 0.0, 1.0], uv: [1.0, 1.0], color: [1.0, 0.0, 0.0] },
            Vertex { position: [hw, hh, hd], normal: [0.0, 0.0, 1.0], uv: [1.0, 0.0], color: [1.0, 0.0, 0.0] },
            Vertex { position: [-hw, hh, hd], normal: [0.0, 0.0, 1.0], uv: [0.0, 0.0], color: [1.0, 0.0, 0.0] },
            
            // Back face (z-)
            Vertex { position: [hw, -hh, -hd], normal: [0.0, 0.0, -1.0], uv: [0.0, 1.0], color: [0.0, 1.0, 0.0] },
            Vertex { position: [-hw, -hh, -hd], normal: [0.0, 0.0, -1.0], uv: [1.0, 1.0], color: [0.0, 1.0, 0.0] },
            Vertex { position: [-hw, hh, -hd], normal: [0.0, 0.0, -1.0], uv: [1.0, 0.0], color: [0.0, 1.0, 0.0] },
            Vertex { position: [hw, hh, -hd], normal: [0.0, 0.0, -1.0], uv: [0.0, 0.0], color: [0.0, 1.0, 0.0] },
            
            // Top face (y+)
            Vertex { position: [-hw, hh, hd], normal: [0.0, 1.0, 0.0], uv: [0.0, 1.0], color: [0.0, 0.0, 1.0] },
            Vertex { position: [hw, hh, hd], normal: [0.0, 1.0, 0.0], uv: [1.0, 1.0], color: [0.0, 0.0, 1.0] },
            Vertex { position: [hw, hh, -hd], normal: [0.0, 1.0, 0.0], uv: [1.0, 0.0], color: [0.0, 0.0, 1.0] },
            Vertex { position: [-hw, hh, -hd], normal: [0.0, 1.0, 0.0], uv: [0.0, 0.0], color: [0.0, 0.0, 1.0] },
            
            // Bottom face (y-)
            Vertex { position: [-hw, -hh, -hd], normal: [0.0, -1.0, 0.0], uv: [0.0, 1.0], color: [1.0, 1.0, 0.0] },
            Vertex { position: [hw, -hh, -hd], normal: [0.0, -1.0, 0.0], uv: [1.0, 1.0], color: [1.0, 1.0, 0.0] },
            Vertex { position: [hw, -hh, hd], normal: [0.0, -1.0, 0.0], uv: [1.0, 0.0], color: [1.0, 1.0, 0.0] },
            Vertex { position: [-hw, -hh, hd], normal: [0.0, -1.0, 0.0], uv: [0.0, 0.0], color: [1.0, 1.0, 0.0] },
            
            // Right face (x+)
            Vertex { position: [hw, -hh, hd], normal: [1.0, 0.0, 0.0], uv: [0.0, 1.0], color: [1.0, 0.0, 1.0] },
            Vertex { position: [hw, -hh, -hd], normal: [1.0, 0.0, 0.0], uv: [1.0, 1.0], color: [1.0, 0.0, 1.0] },
            Vertex { position: [hw, hh, -hd], normal: [1.0, 0.0, 0.0], uv: [1.0, 0.0], color: [1.0, 0.0, 1.0] },
            Vertex { position: [hw, hh, hd], normal: [1.0, 0.0, 0.0], uv: [0.0, 0.0], color: [1.0, 0.0, 1.0] },
            
            // Left face (x-)
            Vertex { position: [-hw, -hh, -hd], normal: [-1.0, 0.0, 0.0], uv: [0.0, 1.0], color: [0.0, 1.0, 1.0] },
            Vertex { position: [-hw, -hh, hd], normal: [-1.0, 0.0, 0.0], uv: [1.0, 1.0], color: [0.0, 1.0, 1.0] },
            Vertex { position: [-hw, hh, hd], normal: [-1.0, 0.0, 0.0], uv: [1.0, 0.0], color: [0.0, 1.0, 1.0] },
            Vertex { position: [-hw, hh, -hd], normal: [-1.0, 0.0, 0.0], uv: [0.0, 0.0], color: [0.0, 1.0, 1.0] },
        ];

        let indices = vec![
            0, 1, 2, 0, 2, 3,       // Front
            4, 5, 6, 4, 6, 7,       // Back
            8, 9, 10, 8, 10, 11,    // Top
            12, 13, 14, 12, 14, 15, // Bottom
            16, 17, 18, 16, 18, 19, // Right
            20, 21, 22, 20, 22, 23, // Left
        ];

        Geometry::new(vertices, indices)
    }
}

