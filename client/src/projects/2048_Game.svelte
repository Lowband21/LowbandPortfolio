<script>
  import { onMount } from "svelte";

  let board = [[]];
  let score = 0;
  let validActions = [];

  async function fetchBoard() {
    const response = await fetch(`/game/new`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
    });
    const data = await response.json();
    board = data.board;
    console.log(board);
  }

  async function performAction(action) {
    const response = await fetch(`/game/move`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ action }),
    });

    const data = await response.json();
    console.log(data);
    board = data.board;
    score = data.score;
    validActions = data.validActions;
  }
  function getColor(value) {
    switch (value) {
      case 2:
        return "#eee4da";
      case 4:
        return "#ede0c8";
      case 8:
        return "#f2b179";
      case 16:
        return "#f59563";
      case 32:
        return "#f67c5f";
      case 64:
        return "#f65e3b";
      case 128:
        return "#edcf72";
      case 256:
        return "#edcc61";
      case 512:
        return "#edc850";
      case 1024:
        return "#edc53f";
      case 2048:
        return "#edc22e";
      default:
        return "#ccc"; // Default grayish color for zero or other unknown values
    }
  }
  let startX, startY, endX, endY;

  function handleStart(event) {
    if (event.touches) {
      // Touch event
      startX = event.touches[0].clientX;
      startY = event.touches[0].clientY;
    } else {
      // Mouse event
      startX = event.clientX;
      startY = event.clientY;
    }
  }

  function handleEnd(event) {
    if (event.changedTouches) {
      // Touch event
      endX = event.changedTouches[0].clientX;
      endY = event.changedTouches[0].clientY;
    } else {
      // Mouse event
      endX = event.clientX;
      endY = event.clientY;
    }

    processSwipeDirection();
  }

  function processSwipeDirection() {
    const deltaX = endX - startX;
    const deltaY = endY - startY;

    if (Math.abs(deltaX) > Math.abs(deltaY)) {
      // Horizontal swipe
      if (deltaX > 0) performAction("MergeRight");
      else performAction("MergeLeft");
    } else {
      // Vertical swipe
      if (deltaY > 0) performAction("MergeDown");
      else performAction("MergeUp");
    }
  }
  let previousBoard = [[]]; // To store the previous state of the board

  $: if (previousBoard && board) {
    applyMergeAnimations();
    previousBoard = JSON.parse(JSON.stringify(board)); // Deep copy
  }

  function applyMergeAnimations() {
    const cells = document.querySelectorAll(".cell");
    for (let i = 0; i < cells.length; i++) {
      const cell = cells[i];
      const prevValue = previousBoard[Math.floor(i / 4)][i % 4];
      const currValue = board[Math.floor(i / 4)][i % 4];

      if (currValue && currValue === 2 * prevValue) {
        // Check if cell merged
        cell.style.animation = "mergeEffect 0.5s";
      }
    }
  }
  onMount(() => {
    fetchBoard();
  });
</script>

<section id="game-description">
  <h2>2048 Game - Svelte & Rust Actix</h2>

  <p><strong>Introduction:</strong></p>
  <p>
    This is a game I have a lot of hours in and I was so excited to take on my own implementation.
    This version is crafted using Svelte for the
    front-end and powered by Rust Actix for the backend logic. However, it's based on a cli version
    that I built during a quarter where I was taking an introductory class on AI. The original version had a cli interface
    and primarly existed as a way to train and test my Q-Learning agent. This version is stripped of that functionality, but has been given a simple GUI.
  </p>

  <h3>How to Play:</h3>
  <p>
    Simply swipe or drag the tiles in your desired direction - left, right, up,
    or down. You can also use the provided buttons. Merge tiles with the same
    number to double their value. The goal? Create a tile with the value 2048.
    Happy gaming!
  </p>

  <div class="links">
    <a href="https://github.com/Lowband21/rusty2048" target="_blank"
      >View the Code</a
    >
  </div>
</section>

<div
  class="board"
  on:mousedown={handleStart}
  on:mouseup={handleEnd}
  on:touchstart={handleStart}
  on:touchend={handleEnd}
>
  {#each board as row}
    {#each row as cell}
      <div class="cell" style="background-color: {getColor(cell)};">{cell}</div>
    {/each}
  {/each}
</div>

<div>
  <strong>Score: </strong>
  {score}
</div>

<div>
  <button on:click={() => performAction("Left")}>Left</button>
  <button on:click={() => performAction("Right")}>Right</button>
  <button on:click={() => performAction("Up")}>Up</button>
  <button on:click={() => performAction("Down")}>Down</button>
</div>

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
    grid-template-columns: repeat(
      4,
      20vw
    ); /* Columns take up 25% of viewport width */
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
    font-family: "Arial", sans-serif; /* Using Arial font, but feel free to change it to your preferred font */
    transition: transform 0.5s ease-out, background-color 0.5s ease-out; /* Add this line */
    position: relative; /* To make sure our absolute positioning inside works */
  }

  /* Keyframes for the merging effect */
  @keyframes mergeEffect {
    from {
      transform: scale(1.1);
    }
    to {
      transform: scale(1);
    }
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
