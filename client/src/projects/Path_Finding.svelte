<script>
  const gridSize = 20;
  let grid = Array(gridSize)
    .fill()
    .map(() => Array(gridSize).fill("Empty"));
  let selectedAlgorithm = "";
  let startPoint = null;
  let endPoint = null;

  function toggleBarrier(x, y) {
    if (grid[x][y] === "Barrier") {
      grid[x][y] = "Empty";
    } else if (grid[x][y] === "Empty") {
      grid[x][y] = "Barrier";
    }
  }

  function setStartOrEnd(x, y, type) {
    if (type === "Start") {
      if (startPoint) grid[startPoint.x][startPoint.y] = "Empty";
      startPoint = { x, y };
      grid[x][y] = "Start";
    } else {
      if (endPoint) grid[endPoint.x][endPoint.y] = "Empty";
      endPoint = { x, y };
      grid[x][y] = "End";
    }
  }

  function clearGrid() {
    grid = Array(gridSize)
      .fill()
      .map(() => Array(gridSize).fill("Empty"));
    startPoint = null;
    endPoint = null;
  }

  import { onMount } from "svelte";

  async function startVisualization() {
    if (!startPoint || !endPoint || !selectedAlgorithm) return;

    const response = await fetch(`/${selectedAlgorithm}`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        grid: grid,
        start: [startPoint.x, startPoint.y],
        end: [endPoint.x, endPoint.y],
      }),
    });

    const responseData = await response.json();
    if (response.ok && responseData && Array.isArray(responseData)) {
      for (const [x, y] of responseData) {
        if (
          x >= 0 &&
          x < grid.length &&
          y >= 0 &&
          y < grid[x].length &&
          grid[x][y] !== "Start" &&
          grid[x][y] !== "End"
        ) {
          grid[x][y] = "Path";
        }
      }
    } else {
      console.error("Pathfinding failed or path not found.");
    }
  }

  onMount(() => {
    // Prevent the default right-click context menu on the entire page
    window.addEventListener("contextmenu", (e) => e.preventDefault());
  });
</script>

<title>Interactive Pathfinding Visualizer</title>
<body>
  <h1>Interactive Pathfinding Visualizer</h1>
  <p>
    Dive deep into the world of pathfinding algorithms with my interactive
    visualizer! Algorithms are fundamental to the essence of computing, and
    pathfinding is a crucial component in many applications, ranging from video
    games to GPS systems. My tool allows you to:
  </p>
  <ul>
    <li>
      <span class="highlight">Create Custom Grids:</span> Easily design your own
      maps with obstacles, and select start and end points.
    </li>
    <li>
      <span class="highlight">Choose Your Algorithm:</span> Learn how different algorithms
      like A*, Dijkstra's, and BFS tackle pathfinding differently.
    </li>
    <li>
      <span class="highlight">Visualize in Real-Time:</span> Watch as the chosen
      algorithm searches its way through the maze, providing a step-by-step visualization
      of its thought process.
    </li>
    <li>
      <span class="highlight">Interactive Learning:</span> Each time the algorithm
      hits an obstacle, takes a wrong turn, or finds the shortest path, you get to
      learn the 'why' behind it.
    </li>
  </ul>
</body>
<div class="controls">
  <select bind:value={selectedAlgorithm}>
    <option value="">Select an algorithm</option>
    <option value="a-star">A*</option>
    <option value="dijkstra">Dijkstra</option>
    <option value="bfs">BFS</option>
  </select>
  <button on:click={clearGrid}>Clear Grid</button>
  <button on:click={startVisualization}>Start Visualization</button>
</div>

<div class="grid" style="--grid-size: {gridSize}">
  {#each grid as row, x}
    {#each row as cell, y}
      <div
        class="cell {cell}"
        on:click={() => toggleBarrier(x, y)}
        on:contextmenu|preventDefault={(event) => {
          if (!startPoint) {
            setStartOrEnd(x, y, "Start");
          } else {
            setStartOrEnd(x, y, "End");
          }
        }}
      />
    {/each}
  {/each}
</div>

<style>
  .grid {
    display: grid;
    grid-template-columns: repeat(var(--grid-size), 1fr);
    grid-auto-rows: calc(
      100% / var(--grid-size)
    );
    gap: 0;
  }
  .cell {
    height: 50px;
    background-color: white;
    border: 1px solid #ddd;
    box-sizing: border-box; 
  }
  .cell.Barrier {
    background-color: black;
    border: none;
  }
  .cell.Start {
    background-color: green;
    border: none;
  }

  .cell.Path {
    background-color: lightblue;
    border: none;
  }
  .cell.End {
    background-color: red;
    border: none;
  }
  body {
    font-family: Arial, sans-serif;
    width: 90%;
    margin: 40px;
    line-height: 1.6;
    box-sizing: border-box;
  }

  h1 {
    color: #ffffff;
    border-bottom: 2px solid #555;
    padding-bottom: 10px;
  }

  p {
    color: #fff;
    margin: 20px 0;
  }

  ul {
    list-style-type: disc;
    margin-left: 40px;
  }

  ul li {
    margin: 10px 0;
    color: #fff;
  }

  .highlight {
    font-weight: bold;
    color: #fff;
  }
</style>
