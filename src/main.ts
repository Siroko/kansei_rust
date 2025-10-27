import init, { Engine } from '../rust-wasm/pkg/rust_wasm.js';

const canvasScale = window.devicePixelRatio;
async function main() {
  const canvas = document.getElementById('webgpu-canvas') as HTMLCanvasElement;
  canvas.width = window.innerWidth * canvasScale;
  canvas.height = window.innerHeight * canvasScale;
  try {
    // Check WebGPU support
    if (!navigator.gpu) {
      throw new Error('WebGPU is not supported in your browser. Please use Chrome/Edge 113+ or Safari 18+');
    }

    // Initialize WASM module
    await init();
    console.log('WASM module initialized');

    // Create engine - everything initializes in Rust!
    const engine = await Engine.new('webgpu-canvas', canvas.width, canvas.height);
   
    // Animation loop - all logic is in Rust!
    let lastFrameTime = performance.now();

    function animate(currentTime: number) {
      try {
        // Calculate delta time (normalized to 60fps baseline)
        const deltaTime = (currentTime - lastFrameTime) / 16.67; // 16.67ms = 60fps
        lastFrameTime = currentTime;

        // Update engine state (animations, physics, etc.) - all in Rust!
        engine.update(deltaTime);
        // Render
        engine.render();

        requestAnimationFrame(animate);
      } catch (err) {
        console.error('Render error:', err);
      }
    }

    // Handle window resize
    window.addEventListener('resize', () => {
      canvas.width = window.innerWidth * canvasScale;
      canvas.height = window.innerHeight * canvasScale;
      engine.set_size(canvas.width, canvas.height);
    });

    // Start animation
    animate(performance.now());

  } catch (err) {
    console.error('Initialization error:', err);
  }
}

main();

