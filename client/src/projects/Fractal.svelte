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
  const baseResolution = { width: 2800, height: 2100 }; // Base resolution
  let resolution = { ...baseResolution };

  // Function to adjust the maximum iterations based on the current zoom level
  function adjustMaxIterations() {
    if (zoom < 0.2) {
      maxIterations = 20000.0;
    } else if (zoom < 0.5) {
      // Linear interpolation between 20000 and 5000 as zoom goes from 0.2 to 0.5
      maxIterations = 20000.0 - (zoom - 0.2) * (15000.0 / 0.3);
    } else {
      // Linear interpolation between 5000 and 1000 as zoom goes from 0.5 to 1.0 (or more)
      maxIterations = 5000.0 - (zoom - 0.5) * (4000.0 / 0.5);
      maxIterations = Math.max(1000.0, maxIterations); // Ensure it doesn't go below 1000
    }
  }

  let tessellationUpdateRequired = true;
  function getTessellationLevel(zoom, pan) {
    console.log(zoom);
    if (zoom < 0.2) return 128; // Use high tessellation for close-up views
    if (Math.abs(pan.x) < 0.5 && Math.abs(pan.y) < 0.5) return 64; // Higher tessellation near the center
    if (zoom < 0.5) return 4;
    if (zoom < 1.0) return 1;
    return 1; // Lowest tessellation for distant views
  }

  function createTessellatedQuads(level) {
    const quads = [];
    const size = 2.0 / level;
    for (let i = 0; i < level; i++) {
      for (let j = 0; j < level; j++) {
        const quad = [
          -1 + i * size,
          -1 + j * size,
          -1 + (i + 1) * size,
          -1 + j * size,
          -1 + i * size,
          -1 + (j + 1) * size,

          -1 + (i + 1) * size,
          -1 + j * size,
          -1 + i * size,
          -1 + (j + 1) * size,
          -1 + (i + 1) * size,
          -1 + (j + 1) * size,
        ];
        quads.push(...quad);
      }
    }
    return quads;
  }

  // Vertex Shader Source
  // This shader simply passes the vertex positions to the fragment shader
  const vs = `
    #version 300 es
    in vec2 position;

    void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    }
  `;

  // Fragment Shader Source for Mandelbrot
  // This shader computes the Mandelbrot set and assigns colors based on the iteration count
  const fs = `
    #version 300 es
    precision mediump float;
    out vec4 outputColor;
    const float TWO = 2.0;
    const float FOUR = 4.0;
    const float COLOR_MULTIPLIER = 90.0;
    uniform vec2 resolution;
    uniform vec2 pan;
    uniform float zoom;
    uniform float maxIterations;
    
vec3 getColor(float iterations) {
    vec3 color;
    float normalized = pow(iterations / maxIterations, 0.3); // Adjust the exponent to tweak the sensitivity

    // Define thresholds for the color bands
    float t1 = 0.1;  
    float t2 = 0.3;  
    float t3 = 0.6;  
    float t4 = 0.8;  

    if (normalized < t1) {
        color = mix(vec3(1.0, 1.0, 1.0), vec3(0.0, 0.0, 1.0), normalized / t1);
    } else if (normalized < t2) {
        color = mix(vec3(0.0, 0.0, 1.0), vec3(1.0, 0.0, 0.0), (normalized - t1) / (t2 - t1));
    } else if (normalized < t3) {
        color = mix(vec3(1.0, 0.0, 0.0), vec3(1.0, 0.5, 0.0), (normalized - t2) / (t3 - t2));
    } else if (normalized < t4) {
        color = mix(vec3(1.0, 0.5, 0.0), vec3(0.0, 0.0, 0.0), (normalized - t3) / (t4 - t3));
    } else {
        color = vec3(0.0, 0.0, 0.0);
    }

    return color;
}
    
    void main() {
        vec2 uv = (gl_FragCoord.xy - 0.5 * resolution) / min(resolution.y, resolution.x);
        uv *= zoom;
        uv += pan;
        vec2 c = uv;
        vec2 z = vec2(0.0);
        float iterations = 0.0;
        for (float i = 0.0; i < maxIterations; i++) {
            if (dot(z, z) > FOUR) break;
            vec2 temp = vec2(z.x*z.x - z.y*z.y, TWO * z.x*z.y) + c;
            z = temp;
            iterations++;
        }
        vec3 color = getColor(iterations);
        outputColor = vec4(color, 1.0);
    }
`;
  let debounceTimeout;

  function debouncedRender() {
    clearTimeout(debounceTimeout);
    debounceTimeout = setTimeout(() => {
      adjustMaxIterations();
      tessellationUpdateRequired = true;
    }, 100);
  }
  // Svelte lifecycle function that runs when the component is mounted
  onMount(async () => {
    // Get the WebGL context from the canvas element
    const gl = document
      .getElementById("canvas")
      .getContext("webgl2", { antialias: true });

    // Create a WebGL program using the provided vertex and fragment shaders
    const programInfo = twgl.createProgramInfo(gl, [vs, fs]);

    // Define the vertex positions for the full-screen quad
    const arrays = {
      position: [-1, -1, 0, 1, -1, 0, -1, 1, 0, -1, 1, 0, 1, -1, 0, 1, 1, 0],
    };

    let previousTessellationLevel = -1;
    let tessellatedBufferInfo; // Declare it outside of the render function

    // Function to render the Mandelbrot set
    function render(time) {
      // Resize the canvas to the adjusted resolution
      gl.canvas.width = resolution.width;
      gl.canvas.height = resolution.height;

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

      // Bind the WebGL program
      gl.useProgram(programInfo.program);

      // Determine the tessellation level based on zoom and generate the tessellated quads
      const tessellationLevel = getTessellationLevel(zoom, pan);
      if (tessellationUpdateRequired) {
        const quads = createTessellatedQuads(tessellationLevel);
        const arrays = {
          position: {
            numComponents: 2,
            data: quads,
          },
        };
        tessellatedBufferInfo = twgl.createBufferInfoFromArrays(gl, arrays);
        tessellationUpdateRequired = false;
      }

      // Bind the WebGL program and set the buffers, attributes, and uniforms
      gl.useProgram(programInfo.program);
      twgl.setBuffersAndAttributes(gl, programInfo, tessellatedBufferInfo);
      twgl.setUniforms(programInfo, uniforms);

      // Draw the Mandelbrot set
      twgl.drawBufferInfo(gl, tessellatedBufferInfo, gl.TRIANGLES);

      // Request the next frame
      requestAnimationFrame(render);
    }

    // Function to handle zooming with the mouse wheel
    function handleWheel(event) {
      const delta = event.deltaY;
      zoom *= delta > 0 ? 1.1 : 0.9;
      debouncedRender();
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
    return () => {
      canvas.removeEventListener("wheel", handleWheel);
      canvas.removeEventListener("mousemove", handleMouseMove);
      window.removeEventListener("wheel", preventScroll);
    };
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
