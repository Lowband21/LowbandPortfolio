<!-- Modal_Editing.svelte -->
<script>
  export let post = {
    title: "Modal Editing: A Productivity Game-Changer",
    content: `
    <h2>Introduction</h2>
    <p>Modal editing, a key feature in certain text editors, is a technique that has dramatically enhanced my coding productivity. I started using Vim, a popular modal text editor, in 2019 and it has since become a crucial part of my workflow.</p>
    
    <h2>Modal Editing and Vim</h2>
    <p>At the heart of Vim's design is the concept of modal editing. Unlike most text editors, which operate in a single mode, Vim has multiple modes, each designed for a specific kind of task. This unique approach makes complex edits and movements a breeze, once you get the hang of it.</p>

    <p>The learning curve is steep, but the reward is high. After years of usage, I can confidently say that Vim's modal editing has made me a more efficient programmer.</p>

    <h2>Modal Editing Beyond Vim</h2>
    <p>My love for modal editing doesn't stop at Vim. I've taken steps to enable modal editing in other areas of my workflow as well. My favorite example is Obsidian, a note-taking application I use frequently. Obsidian has Vim keybinding support, which lets me navigate and edit my notes with the same efficiency as when I'm coding.</p>
    
    <h2>Conclusion</h2>
    <p>In conclusion, modal editing has transformed my coding and note-taking workflow, making me a more productive developer and learner. If you're on the fence about trying out Vim or any other modal text editor, I'd say give it a shot! The learning curve may be steep, but the payoff in productivity is well worth it.</p>
    `,
  };
</script>

<div>
  <h2>{post.title}</h2>
  <div>{@html post.content}</div>
</div>