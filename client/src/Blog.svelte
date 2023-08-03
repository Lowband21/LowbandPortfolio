<!-- Blog.svelte -->
<script>
  // Sample data for blog posts
  const blogPosts = [
    {
      id: 1,
      title: "Getting Started with Svelte",
      content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
    },
    {
      id: 2,
      title: "Understanding Reactive Declarations in Svelte",
      content: "Svelte's reactivity system is amazing! Let's explore it.",
    },
    {
      id: 3,
      title: "Building a Simple To-Do App with Svelte",
      content: "Learn how to build a to-do app using Svelte and local storage.",
    },
  ];
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
  {#each blogPosts as post (post.id)}
    <div class="blog-post">
      <h2 class="blog-title">{post.title}</h2>
      <p>{post.content}</p>
    </div>
  {/each}
</div>
