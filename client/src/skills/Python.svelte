<!-- PostPythonExperience.svelte -->
<script>
  export let post = {
    title: "My Journey with Python: From High School to Professional Development",
    content: `
    <h2>Introduction</h2>
    <p>Python - a versatile, powerful language that has become a staple in my developer toolkit. My journey with Python started in high school, took a backseat during early university days, and then resurfaced stronger than ever, becoming a dominant workhorse in my programming repertoire.</p>
    
    <h2>Early Days: High School Projects</h2>
    <p>In high school, I took my first steps into the world of Python, using it to build a variety of small personal projects. The simplicity of the syntax and the power it offered were immediately appealing, and I enjoyed the process of learning and experimenting with this new tool.</p>

    <h2>A Brief Intermission: University and Java</h2>
    <p>As I entered university, my focus shifted to Java, as my school primarily used it for undergraduate instruction. Python took a backseat during this time, but the foundation I had built in high school remained.</p>

    <h2>Back in the Saddle: Sophomore Year and Beyond</h2>
    <p>My sophomore year marked a turning point. The university transitioned from Java to Python for undergraduate instruction. This change reignited my engagement with Python, and I found myself delving deeper into its capabilities and applications.</p>
    
    <h2>Python as a Workhorse</h2>
    <p>Since then, Python has been an indispensable part of my programming toolkit. I've used it extensively for a wide range of applications, from scripting and automation to integration, artificial intelligence, and web development. The flexibility and power of Python have made it my go-to language for many projects.</p>

    <h2>Conclusion</h2>
    <p>Looking back at my journey, Python has been a constant companion, adapting and growing with me. It's a testament to the versatility and robustness of the language, and I look forward to further adventures with Python in my programming journey.</p>
    `,
  };
</script>

<div>
  <h2>{post.title}</h2>
  <div>{@html post.content}</div>
</div>