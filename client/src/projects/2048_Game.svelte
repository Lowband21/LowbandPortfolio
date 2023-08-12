<script>
  import { onMount } from 'svelte';

  let board = [[]];
  let score = 0;
  let validActions = [];

  async function fetchBoard() {
    const response = await fetch(`/game/new`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
    });
    const data = await response.json();
    board = data.board;
    console.log(board);
  }

  async function performAction(action) {
    const response = await fetch(`/game/move`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({ action })
    });

    const data = await response.json();
    console.log(data);
    board = data.board;
    score = data.score;
    validActions = data.validActions;
  }
  function getColor(value) {
    switch (value) {
      case 2: return "#eee4da";
      case 4: return "#ede0c8";
      case 8: return "#f2b179";
      case 16: return "#f59563";
      case 32: return "#f67c5f";
      case 64: return "#f65e3b";
      case 128: return "#edcf72";
      case 256: return "#edcc61";
      case 512: return "#edc850";
      case 1024: return "#edc53f";
      case 2048: return "#edc22e";
      default: return "#ccc"; // Default grayish color for zero or other unknown values
    }
  }
  let startX, startY, endX, endY;

  function handleMouseDown(event) {
    startX = event.clientX;
    startY = event.clientY;
  }

  function handleMouseUp(event) {
    endX = event.clientX;
    endY = event.clientY;

    // Determine the swipe direction
    const deltaX = endX - startX;
    const deltaY = endY - startY;

    if (Math.abs(deltaX) > Math.abs(deltaY)) { // Horizontal swipe
      if (deltaX > 0) performAction('MergeRight');
      else performAction('MergeLeft');
    } else { // Vertical swipe
      if (deltaY > 0) performAction('MergeDown');
      else performAction('MergeUp');
    }
  }

  onMount(() => {
    fetchBoard();
  });
</script>

<style>
.board {
    display: grid;
    grid-template-columns: repeat(4, 50px); /* Set explicit width for columns */
    gap: 2px; /* Reduced gap */
    margin: 5px; /* Margin around the board */
    width: calc(4 * 50px + 3 * 2px + 2 * 4px); /* Explicitly set the board's width */
    border: 1px solid #aaa; /* A subtle border */
    background-color: #eee; /* Background color */
    padding: 4px; /* Padding around the cells */
}

.cell {
    width: 50px;
    height: 50px;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: #ddd; /* Slightly darker background */
    color: #333; /* Dark text color */
    border-radius: 5px; /* Rounded edges */
    box-shadow: 1px 1px 3px rgba(0, 0, 0, 0.1); /* Depth */
}

button {
    padding: 5px 15px;
    margin: 5px;
    border: none;
    background-color: #007bff;
    color: #fff;
    border-radius: 5px;
    cursor: pointer;
    transition: background-color 0.3s;
}

button:hover {
    background-color: #0056b3;
}

button:disabled {
    background-color: #ccc;
    cursor: not-allowed;
}
</style>

<div class="board" on:mousedown={handleMouseDown} on:mouseup={handleMouseUp}>
  {#each board as row}
    {#each row as cell}
      <div class="cell" style="background-color: {getColor(cell)};">{cell}</div>
    {/each}
  {/each}
</div>

<div>
  <strong>Score: </strong> {score}
</div>

<div>
  <button on:click={() => performAction('MergeLeft')} >Left</button>
  <button on:click={() => performAction('MergeRight')} >Right</button>
  <button on:click={() => performAction('MergeUp')} >Up</button>
  <button on:click={() => performAction('MergeDown')} >Down</button>
</div>