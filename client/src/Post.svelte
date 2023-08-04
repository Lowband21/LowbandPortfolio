<!-- Post.svelte -->
<script>
  import { blogPosts } from "./blog/index.js";
  import { onMount } from "svelte";
  import { Link } from "svelte-routing";

  export let id;

  let Component;

  onMount(async () => {
    Component = blogPosts[id].component;
  });
</script>

<div class="blog-post">
  {#if Component}
    <svelte:component this={Component} />
  {/if}
  <Link to="/blog">Back to blog</Link>
</div>

