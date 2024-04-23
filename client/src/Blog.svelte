<!-- Blog.svelte -->
<script>
  import { blogPosts } from "./blog/index.js";
  import { Link } from "svelte-routing";
  import { flip } from "svelte/animate";

  import "global.css";

  let posts = blogPosts.map((Component, id) => ({ id, Component })).reverse();
</script>

<div class="homepage">
  <div class="blog-list">
    {#each posts as { id, Component: { component, metadata } } (id)}
      <div class="blog-post" animate:flip={{ duration: 500 }}>
        <h2>{metadata.title}</h2>
        <div class="brief">{@html metadata.brief}</div>
        <Link to={"/post/" + id}>Read more...</Link>
      </div>
    {/each}
  </div>
</div>

<style>
  .homepage {
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: flex-start;
  }

  /* Style for blog post list */
  .blog-list {
    margin: 1rem auto;
    justify-self: center;
    width: 90%;
    max-width: 800px; /* Limiting the width for better readability */
  }

  /* Style for individual blog post */
  .blog-post {
    border: 1px solid var(--text-color); /* Using text color for border */
    border-radius: var(--border-radius);
    padding: 1rem;
    margin-bottom: 1rem;
    background-color: var(--secondary-color);
    transition: transform 0.3s ease-in-out, box-shadow 0.3s ease-in-out;
    text-align: center; /* Replacing text-justify for better text alignment */
    color: var(--text-color); /* Using global text color */
  }

  .blog-post:hover {
    transform: scale(1.05); /* Slightly reduced scale for subtlety */
    box-shadow: 0 5px 15px rgba(0, 0, 0, 0.2); /* Softened shadow for a cleaner look */
  }

  .brief {
    color: var(--text-color); /* Consistent text color */
  }

  h1, h2, h3 {
    margin-bottom: 0.5rem; /* Added spacing below headings */
  }

  p {
    margin-bottom: 1rem; /* Ensuring consistent bottom margin for paragraphs */
  }
</style>
