<!-- Post2.svelte -->
<script>
  export let post = {
    title: "My Second Post",
    content: "This is the content of my first post.",
  };
</script>

<div>
  <h2>{post.title}</h2>
  <p>{post.content}</p>
</div>