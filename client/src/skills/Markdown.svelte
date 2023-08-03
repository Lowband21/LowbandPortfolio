<!-- Markdown.svelte -->
<script>
  export let skill = {
    name: "Markdown",
    experience: `
      <h2>Introduction</h2>
      <p>Markdown has been a fundamental part of my tech stack ever since I was introduced to it. The simplicity of the syntax and the power it provides for formatting text with ease make it an excellent tool for any developer.</p>

      <h2>Compatibility with Other Technologies</h2>
      <p>In addition, the compatibility of Markdown with other technologies, such as static site generators, makes it a versatile tool that I have found useful in multiple scenarios.</p>

      <h2>Markdown and Obsidian</h2>
      <p>One of the applications where I use Markdown the most is Obsidian. Obsidian is a powerful knowledge management tool that uses local Markdown files. The flexibility of Obsidian combined with the simplicity of Markdown has helped me create a personal knowledge system that is robust, easy to navigate, and fun to use.</p>

      <h2>Why I Love Markdown</h2>
      <p>Whether I'm drafting a quick note, writing a detailed technical guide, or brainstorming ideas, Markdown is always my go-to. The love I have for Markdown has grown over time, and I find myself constantly learning new tricks to make my workflow even more efficient.</p>
    `,
  };
</script>

<div>
  <h1>{skill.name}</h1>
  {@html skill.experience}
</div>