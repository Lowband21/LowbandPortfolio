<script>
  // Importing the required dependencies from the modules
  import { onMount } from "svelte";
  import * as twgl from "twgl.js";
  import katex from "katex";

  // State variables
  let wasmReady = false; // State to check if WebAssembly is ready; not used in this code
  let mathElement;
  let zoom = 3.0; // Initial zoom level
  let pan = { x: 0.0, y: 0.0 }; // Initial panning offset

  // Constants
  const maxBaseIterations = 1000.0; // Base maximum iterations for Mandelbrot computation
  let maxIterations = maxBaseIterations;

  // Function to adjust the maximum iterations based on the current zoom level
  function adjustMaxIterations() {
    // Adjust the maximum iterations based on the zoom level, but do not exceed 20000
    maxIterations = Math.min(maxBaseIterations / zoom, 25000); // Adjust based on your needs
  }

  // Vertex Shader Source
  // This shader simply passes the vertex positions to the fragment shader
  const vs = `
    attribute vec4 position;

    void main() {
        gl_Position = position;
    }
  `;

  // Fragment Shader Source for Mandelbrot
  // This shader computes the Mandelbrot set and assigns colors based on the iteration count
  const fs = `
    precision mediump float; // Set the precision for floating point numbers
    uniform vec2 resolution; // Screen resolution
    uniform vec2 pan; // Panning offset
    uniform float zoom; // Zoom level
    uniform float maxIterations; // Maximum iterations for Mandelbrot computation

    // Function to get the color based on the iteration count
    vec3 getColor(float iterations) {
        vec3 color;
        float normalized = iterations / maxIterations;
        // Assign RGB values based on the normalized iteration count
        color.r = sin(90.0 * normalized + 0.0) * 0.6 + 0.4;  // Rusty red
        color.g = sin(90.0 * normalized + 4.0) * 0.2 + 0.3;  // Teal adjustment
        color.b = sin(90.0 * normalized + 2.0) * 0.5 + 0.5;  // Teal adjustment
        return color;
    }

    void main() {
        // Calculate the normalized screen coordinates
        vec2 uv = (gl_FragCoord.xy - 0.5 * resolution) / min(resolution.y, resolution.x);
        uv *= zoom;
        uv += pan;

        // Initialize Mandelbrot variables
        vec2 c = uv;
        vec2 z = vec2(0.0);
        float iterations = 0.0;

        // Compute the Mandelbrot set
        for (float i = 0.0; i < 50000.0; i++) {
            if (iterations >= maxIterations || dot(z, z) > 4.0) break;
            vec2 temp = vec2(z.x*z.x - z.y*z.y, 2.0*z.x*z.y) + c;
            z = temp;
            iterations++;
        }

        // Get the color based on the iteration count and set the fragment color
        vec3 color = getColor(iterations);
        gl_FragColor = vec4(color, 1.0);
    }
`;

  // Svelte lifecycle function that runs when the component is mounted
  onMount(async () => {
    // Get the WebGL context from the canvas element
    const gl = document.getElementById("canvas").getContext("webgl");

    // Create a WebGL program using the provided vertex and fragment shaders
    const programInfo = twgl.createProgramInfo(gl, [vs, fs]);

    // Define the vertex positions for the full-screen quad
    const arrays = {
      position: [-1, -1, 0, 1, -1, 0, -1, 1, 0, -1, 1, 0, 1, -1, 0, 1, 1, 0],
    };
    // Create a buffer with the vertex positions
    const bufferInfo = twgl.createBufferInfoFromArrays(gl, arrays);

    // Function to render the Mandelbrot set
    function render(time) {
      // Resize the canvas to the display size
      twgl.resizeCanvasToDisplaySize(gl.canvas);

      // Set the WebGL viewport
      gl.viewport(0, 0, gl.canvas.width, gl.canvas.height);

      // Adjust the maximum iterations based on the current zoom level
      adjustMaxIterations();

      // Define the uniforms for the shaders
      const uniforms = {
        resolution: [gl.canvas.width, gl.canvas.height],
        zoom: zoom,
        pan: [pan.x, pan.y],
        maxIterations: maxIterations,
      };

      // Bind the WebGL program and set the buffers, attributes, and uniforms
      gl.useProgram(programInfo.program);
      twgl.setBuffersAndAttributes(gl, programInfo, bufferInfo);
      twgl.setUniforms(programInfo, uniforms);

      // Draw the Mandelbrot set
      twgl.drawBufferInfo(gl, bufferInfo);

      // Request the next frame
      requestAnimationFrame(render);
    }

    // Function to handle zooming with the mouse wheel
    function handleWheel(event) {
      const delta = event.deltaY;
      zoom *= delta > 0 ? 1.1 : 0.9;
    }

    // Function to handle panning with the mouse
    function handleMouseMove(event) {
      if (event.buttons === 1) {
        pan.x -= (event.movementX / 2000.0) * zoom;
        pan.y += (event.movementY / 2000.0) * zoom;
      }
    }
    // Function to prevent scrolling
    function preventScroll(event) {
      event.preventDefault();
    }

    // Attach event listeners to the canvas
    const canvas = document.getElementById("canvas");
    canvas.addEventListener("wheel", handleWheel);
    canvas.addEventListener("mousemove", handleMouseMove);
    // Attach event listeners to the canvas to disable and enable scrolling
    canvas.addEventListener("mouseenter", () => {
      window.addEventListener("wheel", preventScroll, { passive: false });
    });
    canvas.addEventListener("mouseleave", () => {
      window.removeEventListener("wheel", preventScroll);
    });

    // Start the rendering loop
    requestAnimationFrame(render);
  });
</script>

<!-- Canvas element to display the Mandelbrot set -->
<canvas id="canvas" />
<!-- Explanation of the Mandelbrot Set -->
<div>
  <h2>Mandelbrot Set Explanation</h2>
  <p>
    The Mandelbrot set, named after its discoverer Benoît B. Mandelbrot, is a
    unique set of complex numbers with fascinating properties. Formed by
    iterating the equation "z_n+1 = z_n^2 + c" and checking whether the
    magnitude of ( z ) exceeds a certain value (usually 2) before a given number
    of iterations, the Mandelbrot set exhibits intricate and infinitely detailed
    boundary that exhibits fractal characteristics. Points inside the Mandelbrot
    set do not escape to infinity, while points outside do.
  </p>
  <p>
    Its beauty is not just in its mathematical definition but also in its visual
    representation. When visualized, it produces intricate patterns and designs
    which are mesmerizing and have captivated mathematicians and artists alike.
  </p>

  <h2>Code Explanation</h2>
  <p>
    This code visualizes the Mandelbrot set using WebGL. Here's a breakdown of
    how it works:
  </p>
  <ul>
    <li>
      <strong>Initialization:</strong> The code begins by importing necessary modules
      and setting up initial values, like zoom level and pan offsets.
    </li>
    <li>
      <strong>Shaders:</strong> Two shaders, a vertex shader and a fragment shader,
      are defined. The vertex shader simply passes the vertex positions to the fragment
      shader. The fragment shader does the heavy lifting — it calculates whether
      each pixel is in the Mandelbrot set and assigns a color based on the number
      of iterations taken.
    </li>
    <li>
      <strong>Rendering:</strong> A function `render` is set up to draw the Mandelbrot
      set. It updates the viewport, adjusts the number of iterations based on the
      zoom level, and then uses the shaders to draw the fractal.
    </li>
    <li>
      <strong>User Interaction:</strong> Event listeners are added to the canvas
      to allow users to zoom in/out using the mouse wheel and pan by clicking and
      dragging.
    </li>
  </ul>
  <p>
    Overall, this code provides an interactive way to explore the Mandelbrot
    set's stunning complexity.
  </p>
</div>

<style>
  /* Add the KaTeX CSS for proper styling */
  @import "katex/dist/katex.min.css";
  /* Styling for the canvas element */
  canvas {
    display: block;
    width: 100%;
    height: 60vw;
  }

  /* Styling for the explanation content */
  div {
    font-family: Arial, sans-serif;
    margin: 20px;
  }

  h2 {
    border-bottom: 2px solid #333;
    padding-bottom: 10px;
  }

  p,
  ul {
    margin-bottom: 20px;
  }
</style>
