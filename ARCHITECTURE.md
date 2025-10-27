# Architecture Overview

This engine follows a modular design inspired by [Kansei](https://github.com/Siroko/kansei), with clear separation of concerns.

## 📦 Package Structure

```
rust-wasm/src/
├── lib.rs                      # Main Engine API (wasm-bindgen exports)
│
├── core_engine/                # Core rendering system (renamed to avoid std::core conflict)
│   ├── mod.rs                  # Module exports
│   ├── renderer.rs             # WebGPU renderer
│   ├── scene.rs                # Scene graph
│   └── camera.rs               # Camera with projection/view
│
├── objects/                    # Scene objects
│   ├── mod.rs                  # Module exports
│   └── mesh.rs                 # Mesh (geometry + transform)
│
├── geometries/                 # Geometry primitives
│   ├── mod.rs                  # Module exports
│   ├── geometry.rs             # Base Geometry + Vertex
│   ├── box_geometry.rs         # Box primitive
│   └── plane_geometry.rs       # Plane primitive
│
├── math/                       # Math utilities
│   ├── mod.rs                  # Module exports
│   ├── vector3.rs              # 3D vectors
│   └── matrix4.rs              # 4x4 matrices
│
└── shaders/                    # WGSL shaders
    └── basic.wgsl              # Basic lit shader
```

## 🎨 Design Principles

### 1. **Modular Architecture**
Each component is self-contained in its own module, similar to Kansei's structure:
- `core_engine/` - Rendering pipeline components (named to avoid std::core conflict)
- `objects/` - Scene objects (meshes, lights, etc.)
- `geometries/` - Reusable geometry definitions
- `math/` - Mathematical primitives

### 2. **Clear Separation of Concerns**
- **Engine**: High-level API, orchestrates components
- **Renderer**: Manages WebGPU device, pipelines, rendering
- **Scene**: Manages collection of objects
- **Camera**: View and projection matrices
- **Mesh**: Combines geometry with transforms
- **Geometry**: Pure vertex/index data

### 3. **Kansei-like API**
```typescript
// Kansei API style
const engine = await Engine.new('canvas', 800, 600);
engine.add_box(1, 1, 1, 0, 0, 0);
engine.set_camera_position(0, 0, 5);
engine.render();
```

## 🔄 Rendering Pipeline

```
Engine.render()
    ↓
Renderer.render(scene, camera)
    ↓
For each mesh in scene:
    1. Create GPU buffers (if needed)
    2. Calculate model matrix (pos, rot, scale)
    3. Update uniforms (view-proj, model)
    4. Draw indexed geometry
    ↓
Submit command buffer
```

## 📝 Adding New Features

### New Geometry Type
1. Create `src/geometries/my_geometry.rs`
2. Implement struct with `new()` method returning `Geometry`
3. Export in `src/geometries/mod.rs`
4. Add method to `Engine` in `src/lib.rs`

### New Object Type
1. Create in `src/objects/`
2. Implement transformation methods
3. Add rendering logic in `Renderer`

### New Math Utility
1. Add to `src/math/vector3.rs` or `matrix4.rs`
2. Or create new file and export in `mod.rs`

## 🎯 Key Differences from Original Kansei

| Kansei (TypeScript) | This Engine (Rust) |
|---------------------|-------------------|
| Runs in JavaScript | Compiles to WASM |
| WebGPU API directly | Uses `wgpu` crate |
| Runtime flexibility | Compile-time safety |
| Dynamic typing | Static typing with generics |
| npm packages | Cargo crates |

## 🚀 Benefits of This Structure

1. **Type Safety**: Rust's type system prevents many bugs at compile time
2. **Performance**: WASM near-native performance
3. **Modularity**: Easy to extend and maintain
4. **Cross-platform**: Same code for web and native (via `wgpu`)
5. **Memory Safety**: Rust's ownership system prevents memory leaks

## 📚 Similar to Kansei Components

| Kansei | This Engine |
|--------|-------------|
| `Renderer` | `core_engine::Renderer` |
| `Scene` | `core_engine::Scene` |
| `Camera` | `core_engine::Camera` |
| `Mesh` | `objects::Mesh` |
| `BoxGeometry` | `geometries::BoxGeometry` |
| `Material` | *(Future: separate materials)* |
| `Vector3` | `math::Vector3` |
| `Matrix4` | `math::Matrix4` |

## 🔮 Future Enhancements

- [ ] Material system with custom shaders
- [ ] Texture loading and management
- [ ] Lighting system (point, directional, spot)
- [ ] Post-processing effects
- [ ] Compute shader support
- [ ] Animation system
- [ ] Physics integration
- [ ] GLTF model loading
- [ ] Shadow mapping
- [ ] PBR materials

