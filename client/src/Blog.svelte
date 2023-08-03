<!-- Blog.svelte -->
<script>
  import { blogPosts } from './blog/index.js';
  import { Link } from 'svelte-routing';

  let posts = blogPosts.map((Component, id) => ({ id, Component }));
</script>


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

  .blog-title {
    font-size: 1.5rem;
    margin-bottom: 0.5rem;
  }
</style>


<div class="blog-list">
  {#each posts as { id, Component } (id)}
    <div class="blog-post">
      <svelte:component this={Component} />
      <Link to={"/post/" + id}>Read more...</Link>
    </div>
  {/each}
</div>