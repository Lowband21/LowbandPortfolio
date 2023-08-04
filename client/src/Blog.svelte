<!-- Blog.svelte -->
<script>
  import { blogPosts } from "./blog/index.js";
  import { Link } from "svelte-routing";

  let posts = blogPosts.map((Component, id) => ({ id, Component })).reverse();
</script>

<div class="blog-list">
  {#each posts as { id, Component: { component, metadata } } (id)}
    <div class="blog-post">
      <h2>{metadata.title}</h2>
      <div>{@html metadata.brief}</div>
      <Link to={"/post/" + id}>Read more...</Link>
    </div>
  {/each}
</div>

<style>
  /* Style for blog post list */
  .blog-list {
    margin: 1rem;
  }

  /* Style for individual blog post */
  .blog-post {
    border: 1px solid #ccc;
    border-radius: 5px;
    padding: 1rem;
    margin-bottom: 1rem;
  }
</style>
