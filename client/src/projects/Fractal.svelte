<script>
  import { onMount } from "svelte";
  import init from "../../../fractal_wasm/pkg/fractal_wasm.js";
  import * as twgl from "twgl.js";

  let wasmReady = false;
  let zoom = 1.0;
  let pan = { x: 0.0, y: 0.0 };
  const maxBaseIterations = 10000.0;
  let maxIterations = maxBaseIterations;

  function adjustMaxIterations() {
    maxIterations = Math.min(maxBaseIterations * zoom, 100000); // Adjust based on your needs
  }

  // Vertex Shader Source
  const vs = `
    attribute vec4 position;

    void main() {
        gl_Position = position;
    }
  `;

  // Fragment Shader Source for Mandelbrot
  const fs = `
    precision mediump float;
    uniform vec2 resolution;
    uniform vec2 pan;
    uniform float zoom;
    uniform float maxIterations;

    vec3 getColor(float iterations) {
        vec3 color;
        float normalized = iterations / maxIterations;
        color.r = sin(90.0 * normalized + 0.0) * 0.6 + 0.4;  // Rusty red
        color.g = sin(90.0 * normalized + 4.0) * 0.2 + 0.3;  // Teal adjustment
        color.b = sin(90.0 * normalized + 2.0) * 0.5 + 0.5;  // Teal adjustment
        return color;
    }

    void main() {
        vec2 uv = (gl_FragCoord.xy - 0.5 * resolution) / min(resolution.y, resolution.x);
        uv *= zoom;
        uv += pan;

        vec2 c = uv;
        vec2 z = vec2(0.0);
        float iterations = 0.0;

        for (float i = 0.0; i < 50000.0; i++) {
            if (iterations >= maxIterations || dot(z, z) > 4.0) break;
            vec2 temp = vec2(z.x*z.x - z.y*z.y, 2.0*z.x*z.y) + c;
            z = temp;
            iterations++;
        }

        vec3 color = getColor(iterations);
        gl_FragColor = vec4(color, 1.0);
    }
`;

  onMount(async () => {
    await init();
    wasmReady = true;

    const gl = document.getElementById("canvas").getContext("webgl");
    const programInfo = twgl.createProgramInfo(gl, [vs, fs]);

    const arrays = {
      position: [-1, -1, 0, 1, -1, 0, -1, 1, 0, -1, 1, 0, 1, -1, 0, 1, 1, 0],
    };
    const bufferInfo = twgl.createBufferInfoFromArrays(gl, arrays);

    function render(time) {
      twgl.resizeCanvasToDisplaySize(gl.canvas);
      gl.viewport(0, 0, gl.canvas.width, gl.canvas.height);

      adjustMaxIterations();

      const uniforms = {
        resolution: [gl.canvas.width, gl.canvas.height],
        zoom: zoom,
        pan: [pan.x, pan.y],
        maxIterations: maxIterations,
      };

      gl.useProgram(programInfo.program);
      twgl.setBuffersAndAttributes(gl, programInfo, bufferInfo);
      twgl.setUniforms(programInfo, uniforms);
      twgl.drawBufferInfo(gl, bufferInfo);

      requestAnimationFrame(render);
    }

    function handleWheel(event) {
      const delta = event.deltaY;
      zoom *= delta > 0 ? 1.1 : 0.9;
    }

    function handleMouseMove(event) {
      if (event.buttons === 1) {
        pan.x -= event.movementX / 10000.0;
        pan.y += event.movementY / 10000.0;
      }
    }

    const canvas = document.getElementById("canvas");
    canvas.addEventListener("wheel", handleWheel);
    canvas.addEventListener("mousemove", handleMouseMove);

    requestAnimationFrame(render);
  });
</script>

<canvas id="canvas" />

<style>
  canvas {
    display: block;
    width: 100%;
    height: 90vw;
  }
</style>
