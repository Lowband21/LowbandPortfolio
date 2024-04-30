<script>
  // You might have links to your images or other dynamic data here.
  const screenshots = [
    { id: 1, src: 'https://raw.githubusercontent.com/Lowband21/bevy_mandelbrot/master/screenshot_1.png', alt: 'Mandelbrot Fractal Viewer Screenshot 1' },
    { id: 2, src: 'https://raw.githubusercontent.com/Lowband21/bevy_mandelbrot/master/screenshot_2.png', alt: 'Mandelbrot Fractal Viewer Screenshot 2' }
  ];

  const repositoryUrl = 'https://github.com/Lowband21/bevy_mandelbrot';

</script>

<style>
  img {
    width: 100%;
    max-width: 1000px;
    height: auto;
    display: block;
    margin-top: 10px;
    margin-bottom: 20px;
    align: center;
  }

  h1, h2 {
    color: #e6ecf4;
  }

  code {
    background-color: #161b22;
    padding: 2px 6px;
    border-radius: 4px;
    font-family: monospace;
  }

  ul, p {
    line-height: 1.6;
  }

  a {
    color: #06c;
    text-decoration: none;
  }

  a:hover {
    text-decoration: underline;
  }
</style>

<div id="content">
  <h1>Welcome to the Mandelbrot Fractal Viewer!</h1>
  <p>Have you ever wanted to explore the beautiful complexities of the Mandelbrot set? Our Bevy-powered application lets you dive into its infinite intricacies with ease. Pan, zoom, and experience the mesmerizing patterns that emerge as you navigate through the fractal landscape.</p>
  
  <h2>Check Out These Amazing Views!</h2>
  {#each screenshots as screenshot}
    <img src={screenshot.src} alt={screenshot.alt}>
  {/each}

  <h2>What Can You Do with This Viewer?</h2>
  <ul>
    <li><strong>Panning:</strong> Freely move across the vast fractal landscape.</li>
    <li><strong>Animated Zooming:</strong> Smoothly dive deeper or zoom out, and appreciate every little detail at any scale.</li>
    <li><strong>High-Resolution Rendering:</strong> Every view is rendered in stunning detail, thanks to the power of modern graphics technology.</li>
    <li><strong>Dynamic Coloring:</strong> Watch the fractal come alive with colors that dynamically adjust as you explore.</li>
    <li><strong>Customizable Coloring:</strong> Want to change the color palette? You can! Choose from preset gradients or create your own to see the fractal in a new light.</li>
    <li><strong>Boundaries:</strong> Set exploration limits to ensure you don't get lost in the zoom.</li>
  </ul>

  <h2>Getting Started</h2>
  <p>Ready to start exploring? Here's how you can get the Mandelbrot Fractal Viewer running on your machine:</p>
  <code>git clone {repositoryUrl}</code><br>
  <code>cd mandelbrot-viewer</code><br>
  <code>cargo run --release</code>

  <h2>How to Navigate</h2>
  <p>Exploring the fractal is easy with our intuitive controls:</p>
  <ul>
    <li><strong>Pan:</strong> Click and drag using the left or middle mouse button to move around.</li>
    <li><strong>Zoom:</strong> Use your mouse wheel to zoom in or out. Hold shift for a faster zoom.</li>
    <li><strong>Inspector:</strong> Press the Escape key to toggle the WorldInspectorPlugin for a detailed look at the scene's internals.</li>
  </ul>
</div>
