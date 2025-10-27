# Kansei-like WASM WebGPU Engine

A modular WebGPU rendering engine written in Rust/WASM, inspired by [Kansei](https://github.com/Siroko/kansei). This engine provides a clean API similar to Three.js/Kansei but with all rendering logic running in Rust.

## Features

- 🦀 **Rust WebAssembly**: Complete engine written in Rust, compiled to WASM
- 🎨 **WebGPU Rendering**: Modern GPU API with `wgpu` crate
- 🏗️ **Modular Architecture**: Inspired by Kansei/Three.js design patterns
- 📦 **Scene Graph**: Hierarchical scene management
- 📐 **Math Library**: Vector3, Matrix4 for 3D transformations
- 🎮 **Camera System**: Perspective camera with view/projection matrices
- 🔺 **Geometry System**: Box, Plane, and custom geometries
- 🎭 **Material System**: Shader-based materials with lighting
- ⚡ **Vite**: Lightning-fast development server
- 📘 **TypeScript**: Type-safe API bindings

## Prerequisites

Before you begin, ensure you have the following installed:

- **Node.js** (v18 or higher)
- **Rust** (latest stable version)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **wasm-pack** (for building Rust to WASM)
  ```bash
  curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
  ```

## Browser Requirements

WebGPU is supported in:
- Chrome/Edge 113+
- Safari 18+
- Firefox 134+ (behind flag)

## Getting Started

1. **Install dependencies**:
   ```bash
   npm install
   ```

2. **Build the WASM module**:
   ```bash
   npm run build:wasm
   ```

3. **Start the development server**:
   ```bash
   npm run dev
   ```

4. Open your browser to the URL shown (typically http://localhost:5173)

## Project Structure

```
wasm/
├── rust-wasm/          # Rust WASM module
│   ├── src/
│   │   └── lib.rs      # Main Rust code with WebGPU rendering
│   ├── Cargo.toml      # Rust dependencies
│   └── pkg/            # Generated WASM output (after build)
├── src/
│   └── main.ts         # TypeScript entry point
├── index.html          # HTML template
├── vite.config.ts      # Vite configuration
└── package.json        # Node.js dependencies

```

## Building for Production

```bash
# Build WASM module
npm run build:wasm

# Build the web app
npm run build

# Preview production build
npm run preview
```

## Architecture

The engine follows a modular design inspired by [Kansei](https://github.com/Siroko/kansei):

### Core Components

```
rust-wasm/src/
├── lib.rs              - Main Engine API exposed to JavaScript
├── core_engine/        - Core rendering components
│   ├── renderer.rs     - WebGPU renderer with pipeline management
│   ├── scene.rs        - Scene graph for managing meshes
│   └── camera.rs       - Camera with projection/view matrices
├── objects/            - Scene objects
│   └── mesh.rs         - Mesh combining geometry and transforms
├── geometries/         - Geometry definitions
│   ├── geometry.rs     - Base Geometry class
│   ├── box_geometry.rs - Box primitive
│   └── plane_geometry.rs - Plane primitive
├── math/               - Math utilities
│   ├── vector3.rs      - 3D vector math
│   └── matrix4.rs      - 4x4 matrix math
└── shaders/
    └── basic.wgsl      - Basic vertex/fragment shaders
```

### How It Works

1. **Engine** (`lib.rs`): High-level API similar to Kansei
   ```rust
   Engine::new() -> Creates renderer, scene, camera
   add_box() -> Adds a box mesh to scene
   rotate_mesh() -> Transforms mesh
   render() -> Renders entire scene
   ```

2. **Renderer** (`renderer.rs`): Manages WebGPU device, pipelines, uniforms
   - Initializes WebGPU device and surface
   - Creates render pipeline with shaders
   - Handles uniform buffers for transforms
   - Renders all meshes in the scene

3. **Scene** (`scene.rs`): Manages collection of meshes
   - Add/remove meshes
   - Iterates for rendering

4. **Camera** (`camera.rs`): View and projection
   - Perspective projection matrix
   - Look-at view matrix
   - Position and aspect ratio

5. **Mesh** (`mesh.rs`): Combines geometry with transforms
   - Position, rotation, scale
   - Model matrix calculation
   - Vertex/index buffers

6. **Geometry** (`geometry.rs`): Vertex data
   - Predefined: Box, Plane
   - Vertex format: position, normal, uv, color
   - Index buffers for efficient rendering

7. **TypeScript** (`src/main.ts`): Minimal JS wrapper
   - Loads WASM module
   - Calls engine methods
   - Animation loop

**Key Advantage**: Using `wgpu` means the same Rust code can run on native platforms (Windows, Mac, Linux) AND web (via WebGPU)!

## API Usage

The engine provides a clean, high-level API:

```typescript
import init, { Engine } from './rust-wasm/pkg/rust_wasm.js';

await init();

// Create engine
const engine = await Engine.new('canvas-id', 800, 600);

// Setup scene
engine.set_camera_position(0, 2, 5);

// Add meshes
const boxIndex = engine.add_box(1.0, 1.0, 1.0, 0, 0, 0);  // returns mesh index
const planeIndex = engine.add_plane(10.0, 10.0, 0, -1, 0);

// Configure meshes
engine.set_mesh_scale(boxIndex, 1.5, 1.5, 1.5);
engine.set_clear_color(0.1, 0.1, 0.15, 1.0);

// Animation loop
function animate() {
  engine.rotate_mesh(boxIndex, 0.01, 0.02, 0.0);
  engine.render();
  requestAnimationFrame(animate);
}
animate();
```

## Extending the Engine

### Add Custom Geometry

Create `rust-wasm/src/geometries/sphere_geometry.rs`:

```rust
use super::{Geometry, Vertex};

pub struct SphereGeometry;

impl SphereGeometry {
    pub fn new(radius: f32, segments: u32) -> Geometry {
        // Generate sphere vertices and indices
        let vertices = vec![...];
        let indices = vec![...];
        Geometry::new(vertices, indices)
    }
}
```

Then add to `rust-wasm/src/geometries/mod.rs`:
```rust
pub mod sphere_geometry;
pub use sphere_geometry::SphereGeometry;
```

### Custom Shaders

Add new shaders in `rust-wasm/src/shaders/`:

```wgsl
// custom.wgsl
@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    // Custom vertex shader
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Custom fragment shader
}
```

### Material System (Future)

The architecture supports adding a material system similar to Kansei's:

```rust
pub struct Material {
    shader: ShaderModule,
    uniforms: Vec<Uniform>,
    transparent: bool,
}
```

## Troubleshooting

**"WebGPU is not supported"**:
- Ensure you're using a compatible browser (Chrome 113+, Safari 18+)
- Check that hardware acceleration is enabled

**WASM module not found**:
- Run `npm run build:wasm` to build the Rust module
- Check that `rust-wasm/pkg/` directory exists

**Build errors**:
- Ensure Rust and wasm-pack are properly installed
- Try `rustup update` to update Rust

## Learn More

- [WebGPU Specification](https://www.w3.org/TR/webgpu/)
- [wasm-bindgen Documentation](https://rustwasm.github.io/docs/wasm-bindgen/)
- [Vite Documentation](https://vitejs.dev/)
- [Rust WebAssembly Book](https://rustwasm.github.io/docs/book/)

