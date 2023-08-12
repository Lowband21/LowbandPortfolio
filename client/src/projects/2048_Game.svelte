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


  function handleStart(event) {
    if (event.touches) { // Touch event
      startX = event.touches[0].clientX;
      startY = event.touches[0].clientY;
    } else { // Mouse event
      startX = event.clientX;
      startY = event.clientY;
    }
  }

  function handleEnd(event) {
    if (event.changedTouches) { // Touch event
      endX = event.changedTouches[0].clientX;
      endY = event.changedTouches[0].clientY;
    } else { // Mouse event
      endX = event.clientX;
      endY = event.clientY;
    }

    processSwipeDirection();
  }

  function processSwipeDirection() {
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
body {
    display: flex;
    flex-direction: column;
    align-items: center; /* Center the content horizontally */
    justify-content: center; /* Center the content vertically */
    height: 100vh; /* Ensure body takes the full viewport height */
}

.board {
    display: grid;
    margin: auto;
    grid-template-columns: repeat(4, 20vw); /* Columns take up 25% of viewport width */
    gap: 0.5vw; /* Use viewport width for gap */
    width: 80vw; /* Board takes up the full viewport width */
    height: 45vh; /* Board takes up half the viewport height */
    border: 0.5vw solid #aaa; 
    background-color: #eee;
    padding: 1vw; 
}

.cell {
    display: flex;
    margin: auto;
    align-items: center;
    justify-content: center;
    background-color: #ddd;
    color: #333;
    border-radius: 2vw; /* Adjusted relative border radius */
    box-shadow: 0.2vw 0.2vw 0.6vw rgba(0, 0, 0, 0.1);
    width: 80%;
    height: 80%;
    font-size: 4vh; /* Adjust this value as per your liking, current value scales with viewport height */
    font-family: 'Arial', sans-serif; /* Using Arial font, but feel free to change it to your preferred font */
}

button {
    padding: 1vh 3vw; /* Adjusted relative padding for button */
    margin: 1vh;
    border: none;
    background-color: #007bff;
    color: #fff;
    border-radius: 1vh;
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

<div class="board" 
     on:mousedown={handleStart} 
     on:mouseup={handleEnd} 
     on:touchstart={handleStart} 
     on:touchend={handleEnd}>
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