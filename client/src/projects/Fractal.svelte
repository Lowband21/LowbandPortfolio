<script>
  import Panzoom from "@panzoom/panzoom";
  import init, {
    compute_fractal,
  } from "../../../fractal_wasm/pkg/fractal_wasm.js";

  let canvas;
  const width = 800; // Set as per your requirement
  const height = 600; // Set as per your requirement
  let zoomLevel = 2;
  let offsetX = 0;
  let offsetY = 0;

  function updateFractal() {
    const pixels = compute_fractal(offsetX, offsetY, zoomLevel, width, height);
    const ctx = canvas.getContext("2d");
    ctx.clearRect(0, 0, width, height); // Clear the canvas before redrawing
    for (let pixel of pixels) {
      ctx.fillStyle = pixel.color;
      ctx.fillRect(pixel.x, pixel.y, 1, 1);
    }
  }

  async function drawFractal() {
    console.log("Drawing Mandelbrot");
    updateFractal();

    const instance = Panzoom(canvas, {
      maxScale: 10, // Set a limit for max zoom level if desired
      minScale: 0.1, // Set a limit for min zoom level if desired
      onPan: (e) => {
        offsetX += e.dx / width;
        offsetY += e.dy / height;
        updateFractal();
      },
      onZoom: (e) => {
        zoomLevel *= e.scale;
        updateFractal();
      },
    });

    // Explicitly enable zooming with the mouse wheel
    canvas.addEventListener("wheel", instance.zoomWithWheel);
  }

  async function run() {
    console.log("Running");
    await init();
    drawFractal();
  }

  run();
</script>

<canvas bind:this={canvas} {width} {height} />
