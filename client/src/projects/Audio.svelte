<script>
  // Array of screenshot links for the visualizer
  const screenshots = [
    { id: 1, src: 'https://raw.githubusercontent.com/Lowband21/bevy_audioviz/master/assets/screenshot_bar.png', alt: 'Bevy Audioviz Bar Visualizer Screenshot' },
    { id: 2, src: 'https://raw.githubusercontent.com/Lowband21/bevy_audioviz/master/assets/screenshot_string.png', alt: 'Bevy Audioviz String Visualizer Screenshot' },
    { id: 3, src: 'https://raw.githubusercontent.com/Lowband21/bevy_audioviz/master/assets/screenshot_circle_split.png', alt: 'Bevy Audioviz Circle Split Visualizer Screenshot' }
  ];

  const repositoryUrl = 'https://github.com/Lowband21/bevy_audioviz';
</script>

<style>
  img {
    width: 100%;
    max-width: 800px;
    height: auto;
    display: block;
    margin: 10px auto;
  }

  h1, h2 {
    color: #f0f8ff;
  }

  code {
    background-color: #282c34;
    color: #f8f8f2;
    padding: 4px 8px;
    border-radius: 6px;
    font-family: monospace;
  }

  ul, p {
    line-height: 1.6;
    color: #dcdcdc;
  }

  a {
    color: #8aecff;
    text-decoration: none;
  }

  a:hover {
    text-decoration: underline;
  }
</style>

<div id="content">
  <h1>Welcome to Bevy Audioviz!</h1>
  <p>Unleash the power of real-time audio visualization with Bevy Audioviz, built using the Bevy engine and Rust for unmatched performance and low latency. Experience your music visually like never before!</p>
  
  <h2>Stunning Visualizations</h2>
  {#each screenshots as screenshot}
    <img src={screenshot.src} alt={screenshot.alt}>
  {/each}

  <h2>Key Features</h2>
  <ul>
    <li><strong>Multiple Visualizers:</strong> Choose between bar, string, or circle visualizations to match your mood or music.</li>
    <li><strong>Real-time Audio Capture:</strong> Instantly capture audio from any configured input or output device.</li>
    <li><strong>Low Latency:</strong> Enjoy real-time response with less than 0.5ms latency per frame with VSync disabled.</li>
    <li><strong>Customizable Visual Experience:</strong> Adjust visualization parameters like color, frequency range, and smoothing through a user-friendly GUI.</li>
  </ul>

  <h2>Getting Started</h2>
  <p>To start visualizing your audio, clone and run the project:</p>
  <code>git clone {repositoryUrl}</code><br>
  <code>cd bevy_audioviz</code><br>
  <code>cargo run</code>

  <h2>Interactive Controls</h2>
  <p>Control your visualization experience with simple keybindings:</p>
  <ul>
    <li><strong>Space:</strong> Switch between different visualizations.</li>
    <li><strong>V:</strong> Toggle VSync for smoother or more responsive visuals.</li>
    <li><strong>G:</strong> Toggle the GUI to customize visualization settings on the fly.</li>
  </ul>
</div>
