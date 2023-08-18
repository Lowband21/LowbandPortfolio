<script>
  import { onMount } from "svelte";
  import Panzoom from "@panzoom/panzoom";
  import { onDestroy } from "svelte";

  let x = 0;
  let y = 0;
  let zoom = 1;
  let targetZoom = 1;
  let canvas;
  let timeout;
  let chunkCache = {};

  function updateZoom() {
    const zoomSpeed = 0.05;
    zoom += (targetZoom - zoom) * zoomSpeed;

    if (Math.abs(targetZoom - zoom) < 0.01) {
      zoom = targetZoom; // Snap to target when close enough
    } else {
      requestAnimationFrame(updateZoom);
    }
    debounceFetchFractal();
  }

  onDestroy(() => {
    window.removeEventListener("resize", resizeCanvas);
  });

  onMount(() => {
    window.addEventListener("resize", resizeCanvas);

    const instance = Panzoom(canvas, {
      onPan: (e) => {
        x -= e.dx / (canvas.width * 2 * zoom);
        y -= e.dy / (canvas.height * 2 * zoom);
        console.log("Updating x and y");
        //debounceFetchFractal();
        // Reset panzoom's transforms so they don't interfere with our rendering
        //instance.reset();
      },
      beforeWheel: (e) => {
        e.preventDefault();

        const zoomFactor = e.deltaY > 0 ? 1.01 : 0.99;
        zoom *= zoomFactor;
        targetZoom *= zoomFactor;

        const rect = canvas.getBoundingClientRect();
        const mouseX = e.clientX - rect.left;
        const mouseY = e.clientY - rect.top;

        const deltaX = (mouseX / canvas.width) * 2 - 1;
        const deltaY = (mouseY / canvas.height) * 2 - 1;

        x += (deltaX * (1 - zoomFactor)) / zoom;
        y += (deltaY * (1 - zoomFactor)) / zoom;

        instance.zoomTo(e.clientX, e.clientY, 1); // reset scale
        updateZoom();
        return true;
      },
    });

    resizeCanvas();
    fetchFractal();
  });

  function resizeCanvas() {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    debounceFetchFractal();
  }

  function renderCachedChunks() {
    const ctx = canvas.getContext("2d");
    for (const key in chunkCache) {
      const cachedData = chunkCache[key];
      const scale = cachedData.zoom / zoom;
      ctx.drawImage(
        cachedData.canvas,
        cachedData.offsetX,
        cachedData.offsetY,
        cachedData.width * scale,
        cachedData.height * scale
      );
    }
  }

  function debounceFetchFractal() {
    if (timeout) clearTimeout(timeout);
    renderCachedChunks();
    timeout = setTimeout(fetchFractal, 50);
  }

  async function fetchFractal() {
    try {
      // Log the values of zoom, x, and y to check assumptions
      console.log("Value of zoom:", zoom);
      console.log("Value of x:", x);
      console.log("Value of y:", y);

      const chunkWidth = Math.ceil(canvas.width / 2);
      const chunkHeight = Math.ceil(canvas.height / 2);

      const chunkPromises = [];
      const baseCoordinates = [-1, 1];

      // Fetch all chunks in parallel
      for (let i = 0; i < baseCoordinates.length; i++) {
        for (let j = 0; j < baseCoordinates.length; j++) {
          const x = baseCoordinates[j] / zoom;
          const y = baseCoordinates[i] / zoom;
          chunkPromises.push(
            fetch(
              `/fractal?x=${x}&y=${y}&zoom=${zoom}&width=${chunkWidth}&height=${chunkHeight}`
            )
          );
        }
      }

      const responses = await Promise.all(chunkPromises);
      const dataChunks = await Promise.all(responses.map((res) => res.json()));

      // Render all chunks
      for (let i = 0; i < dataChunks.length; i++) {
        renderFractal(
          dataChunks[i],
          (i % 2) * chunkWidth,
          Math.floor(i / 2) * chunkHeight
        );
      }
    } catch (error) {
      console.error("Failed to fetch the fractal:", error);
    }
  }

  // Modify the renderFractal to accept x and y offsets for chunks:
  function renderFractal(data, offsetX = 0, offsetY = 0) {
    const ctx = canvas.getContext("2d");

    const imageData = ctx.createImageData(data.width, data.height);
    for (const pixel of data.pixels) {
      const index = (pixel.y * data.width + pixel.x) * 4;
      const color = parseRGB(pixel.color);
      imageData.data[index] = color.r;
      imageData.data[index + 1] = color.g;
      imageData.data[index + 2] = color.b;
      imageData.data[index + 3] = 255;
    }

    ctx.putImageData(imageData, offsetX, offsetY);

    // Cache the rendered chunk
    const offscreenCanvas = document.createElement("canvas");
    offscreenCanvas.width = data.width;
    offscreenCanvas.height = data.height;
    const offscreenCtx = offscreenCanvas.getContext("2d");
    offscreenCtx.putImageData(imageData, 0, 0);
    chunkCache[`${offsetX},${offsetY}`] = {
      canvas: offscreenCanvas,
      offsetX: offsetX,
      offsetY: offsetY,
      width: data.width,
      height: data.height,
      zoom: zoom,
    };
  }

  function parseRGB(rgbString) {
    const matches = rgbString.match(/rgb\((\d+),\s*(\d+),\s*(\d+)\)/);
    return {
      r: parseInt(matches[1]),
      g: parseInt(matches[2]),
      b: parseInt(matches[3]),
    };
  }
</script>

<canvas bind:this={canvas} />

<style>
  canvas {
    transform: none !important;
    display: block;
    width: 100%;
    height: 100%;
    image-rendering: optimizeSpeed;
    image-rendering: -moz-crisp-edges;
    image-rendering: -webkit-optimize-contrast;
    image-rendering: -o-crisp-edges;
    image-rendering: pixelated;
    -ms-interpolation-mode: nearest-neighbor;
  }
</style>
