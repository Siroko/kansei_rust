# Kansei-inspired WebGPU Engine in Rust/WASM

A high-performance modular WebGPU rendering engine written in Rust/WASM, inspired by [Kansei](https://github.com/Siroko/kansei). All rendering logic, animations, and camera controls run in Rust with a minimal JavaScript footprint.

## ✨ Features

- 🦀 **Pure Rust Engine**: Complete 3D engine logic in Rust, compiled to WASM
- 🎨 **WebGPU Rendering**: Modern GPU API using the `wgpu` crate
- 🏗️ **Modular Architecture**: Clean separation inspired by Kansei/Three.js
- 📦 **Scene Graph**: Hierarchical scene management with transforms
- 🎮 **Orbital Camera Controls**: Full mouse/touch camera interaction (ported from Kansei)
- 🎬 **Rust-side Animation**: All animation logic runs in Rust, zero JS overhead
- 📐 **Math Library**: Vector3, Matrix4 with `glam` for optimal performance
- 🔺 **Geometry System**: Box, Plane geometries with extensible architecture
- ⚡ **Fast Development**: Vite with hot reload + Rust file watcher
- 📘 **TypeScript**: Type-safe WASM bindings
- 🎯 **Per-Mesh Uniforms**: Efficient GPU buffer management

## 🚀 Demo

Current demo renders **10,000 animated cubes** at ~55 FPS with:
- Wave-based position animation
- Individual cube rotations
- Orbital camera controls with mouse/touch
- Depth testing and proper z-sorting

## 📋 Prerequisites

- **Node.js** (v18+)
- **Rust** (latest stable)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **wasm-pack**
  ```bash
  curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
  ```

## 🌐 Browser Requirements

WebGPU is supported in:
- ✅ Chrome/Edge 113+
- ✅ Safari 18+
- 🚧 Firefox 134+ (behind flag)

## 🏁 Quick Start

```bash
# Install dependencies
npm install

# Build WASM module
npm run build:wasm

# Start dev server
npm run dev
```

Open http://localhost:5173 in a WebGPU-compatible browser.

## 🔧 Development Workflow

For the best development experience, run both watchers in separate terminals:

**Terminal 1 - Watch Rust files:**
```bash
npm run watch:wasm
```

**Terminal 2 - Run dev server:**
```bash
npm run dev
```

Now any changes to `.rs` files will automatically rebuild WASM and hot-reload the browser!

## 📁 Project Structure

```
wasm/
├── rust-wasm/              # Rust WASM engine
│   ├── src/
│   │   ├── lib.rs          # Main Engine API (public interface)
│   │   ├── core_engine/    # 🎮 Core rendering components
│   │   │   ├── renderer.rs     # WebGPU renderer with pipeline
│   │   │   ├── scene.rs        # Scene graph management
│   │   │   ├── camera.rs       # Camera with proj/view matrices
│   │   │   └── camera_controls.rs  # Orbital camera controls (from Kansei)
│   │   ├── objects/        # 📦 Scene objects
│   │   │   └── mesh.rs         # Mesh with transforms & buffers
│   │   ├── geometries/     # 🔺 Geometry primitives
│   │   │   ├── geometry.rs     # Base geometry trait
│   │   │   ├── box_geometry.rs # Cube primitive
│   │   │   └── plane_geometry.rs
│   │   ├── math/           # 📐 Math utilities (using glam)
│   │   │   ├── vector3.rs
│   │   │   └── matrix4.rs
│   │   └── shaders/        # 🎨 WGSL shaders
│   │       └── basic.wgsl      # Vertex/fragment shaders
│   ├── Cargo.toml
│   └── pkg/                # Generated WASM output
├── src/
│   └── main.ts             # TypeScript entry (minimal)
├── package.json
└── vite.config.ts
```

## 🏗️ Architecture

### Public API (Exposed to JavaScript)

The engine exposes only 3 methods to JavaScript for maximum encapsulation:

```typescript
const engine = await Engine.new('canvas-id', width, height);

// Called every frame
engine.update(deltaTime);  // Update animations, physics
engine.render();           // Render the scene

// Called on window resize
engine.set_size(width, height);
```

### Internal Architecture

Everything else runs in Rust:

**Engine** (`lib.rs`)
- Initializes renderer, scene, camera controls
- Manages animation loop state
- Creates default scene (10,000 cube grid)
- Updates all mesh transforms

**Renderer** (`core_engine/renderer.rs`)
- WebGPU device & pipeline initialization
- Per-mesh uniform buffers & bind groups
- Depth testing configuration
- Command encoding & submission

**Scene** (`core_engine/scene.rs`)
- Manages mesh collection
- Add/remove operations
- Iteration for rendering

**Camera** (`core_engine/camera.rs`)
- Perspective projection matrix
- View matrix with look-at
- Aspect ratio management

**CameraControls** (`core_engine/camera_controls.rs`)
- Orbital camera system (ported from Kansei)
- Mouse drag to rotate
- Mouse wheel to zoom
- Touch gesture support
- Smooth interpolation

**Mesh** (`objects/mesh.rs`)
- Position, rotation, scale transforms
- Model matrix calculation (using `glam`)
- Vertex/index buffer management
- Per-mesh uniform buffer

**Geometry** (`geometries/`)
- Vertex data (position, normal, uv, color)
- Index buffers for efficient rendering
- Extensible for custom geometries

## 💻 Usage Example

```typescript
import init, { Engine } from '../rust-wasm/pkg/rust_wasm.js';

// Initialize WASM
await init();

// Create engine with canvas
const canvas = document.getElementById('webgpu-canvas') as HTMLCanvasElement;
canvas.width = window.innerWidth;
canvas.height = window.innerHeight;

const engine = await Engine.new('webgpu-canvas', canvas.width, canvas.height);

// Animation loop - all logic in Rust!
let lastTime = performance.now();

function animate(currentTime: number) {
  const deltaTime = (currentTime - lastTime) / 16.67; // Normalized to 60fps
  lastTime = currentTime;
  
  engine.update(deltaTime);  // Update animations in Rust
  engine.render();            // Render scene
  
  requestAnimationFrame(animate);
}

animate(performance.now());

// Handle resize
window.addEventListener('resize', () => {
  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;
  engine.set_size(canvas.width, canvas.height);
});
```

## 🎮 Camera Controls

Mouse/touch controls are fully implemented in Rust:

- **Left click + drag**: Rotate camera around target
- **Mouse wheel**: Zoom in/out
- **Touch drag**: Rotate camera
- **Pinch**: Zoom (coming soon)

All ported from [Kansei's CameraControls.ts](https://github.com/Siroko/kansei/blob/main/src/controls/CameraControls.ts).

## ⚡ Performance

Current implementation with 10,000 cubes:
- **~55 FPS** on 120Hz displays
- **CPU-bound**: Limited by 10,000 individual draw calls
- **Next step**: Instanced rendering (1 draw call → 1000+ FPS)

### Why WASM vs Pure JavaScript?

Even though the bottleneck is GPU command submission (language-agnostic), WASM provides:

✅ **Faster computation** (2-10x): Animation calculations, matrix math  
✅ **Predictable performance**: No GC pauses, consistent frame times  
✅ **Memory efficiency**: Dense data structures, better cache locality  
✅ **Type safety**: Compile-time guarantees as codebase scales  

The API boundary overhead is negligible compared to these benefits.

## 🔨 Extending the Engine

### Add Custom Geometry

Create `rust-wasm/src/geometries/sphere_geometry.rs`:

```rust
use super::{Geometry, Vertex};

pub struct SphereGeometry;

impl SphereGeometry {
    pub fn new(radius: f32, segments: u32, rings: u32) -> Geometry {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        
        // Generate sphere vertices
        for ring in 0..=rings {
            // ... vertex generation logic
        }
        
        Geometry::new(vertices, indices)
    }
}
```

Update `rust-wasm/src/geometries/mod.rs`:
```rust
pub mod sphere_geometry;
pub use sphere_geometry::SphereGeometry;
```

### Custom Shaders

Add shaders in `rust-wasm/src/shaders/custom.wgsl`:

```wgsl
struct Uniforms {
    view_proj: mat4x4<f32>,
    model: mat4x4<f32>,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(@location(0) position: vec3<f32>) -> VertexOutput {
    var out: VertexOutput;
    out.position = uniforms.view_proj * uniforms.model * vec4(position, 1.0);
    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(1.0, 0.0, 0.0, 1.0);  // Red
}
```

Load in `renderer.rs`:
```rust
let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
    label: Some("Custom Shader"),
    source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/custom.wgsl").into()),
});
```

## 📦 Building for Production

```bash
# Build WASM module (optimized)
npm run build:wasm

# Build the web app
npm run build

# Preview production build
npm run preview
```

The production build is optimized with:
- LTO (Link-Time Optimization)
- Opt-level "z" (smallest binary)
- wasm-opt optimization

## 🐛 Troubleshooting

**"WebGPU is not supported"**
- Use Chrome 113+, Safari 18+, or Edge 113+
- Enable hardware acceleration in browser settings

**WASM module not found**
- Run `npm run build:wasm` first
- Check `rust-wasm/pkg/` directory exists

**Build errors**
- Update Rust: `rustup update stable`
- Clean build: `cd rust-wasm && cargo clean`

**Black screen**
- Check browser console for WebGPU errors
- Verify canvas element exists with correct ID

**Performance issues**
- Current: 10,000 meshes = ~55 FPS (draw call bound)
- Solution: Instanced rendering (coming soon)

## 🔮 Roadmap

- [ ] **Instanced rendering** (1000+ FPS for particle systems)
- [ ] **Frustum culling** (render only visible objects)
- [ ] **Material system** (custom shaders per mesh)
- [ ] **Lighting** (point, directional, spot lights)
- [ ] **Shadows** (shadow mapping)
- [ ] **Post-processing** (bloom, SSAO, etc.)
- [ ] **Physics integration** (Rapier.js)
- [ ] **GLTF model loading**

## 📚 Resources

- [WebGPU Specification](https://www.w3.org/TR/webgpu/)
- [wgpu Documentation](https://wgpu.rs/)
- [wasm-bindgen Book](https://rustwasm.github.io/docs/wasm-bindgen/)
- [Kansei Engine](https://github.com/Siroko/kansei) (TypeScript inspiration)
- [Rust WebAssembly Book](https://rustwasm.github.io/docs/book/)
- [glam Math Library](https://github.com/bitshifter/glam-rs)

## 📄 License

MIT

## 🙏 Acknowledgments

- [Kansei](https://github.com/Siroko/kansei) for architectural inspiration
- `wgpu` team for excellent WebGPU implementation
- Rust WASM working group for tooling

---

**Built with** 🦀 Rust + ⚡ WebGPU + 🚀 WASM
