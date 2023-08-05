<!-- Blog.svelte -->
<script>
  import { blogPosts } from "./blog/index.js";
  import { Link } from "svelte-routing";
  import { flip } from "svelte/animate";

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
  }

  /* Style for blog post list */
  .blog-list {
    margin: 1rem auto;
    justify-self: center;
    width: 90%;

    
  }

  /* Style for individual blog post */
  .blog-post {
    border: 1px solid #000000;
    border-radius: 5px;
    padding: 1rem;
    margin-bottom: 1rem;
    background-color: #333333;
    transition: all 0.3s ease-in-out;
    text-justify: center;
  }

  .blog-post:hover {
    transform: scale(1.1);
    box-shadow: 0 10px 20px rgba(0, 0, 0, 0.19), 0 6px 6px rgba(0, 0, 0, 0.23);
  }
  .brief {
    color: #ffffff;
  }
  h1, h2, h3, p {
    color: #ffffff;
  }
</style>
