<script>
  import { onMount, onDestroy } from "svelte";
  import { Router, Route, Link } from 'svelte-routing';
  import Home from './Home.svelte';
  import Blog from './Blog.svelte';
  import Projects from './Projects.svelte';
  import Post from './Post.svelte';
  import Project from './Project.svelte'; // New line
  import Skill from './Skill.svelte'; // New line
  import Python from './skills/Python.svelte';
  import Modal_Editing from "./skills/Modal_Editing.svelte";
  import Markdown from "./skills/Markdown.svelte";

  let y = 0;

  const handleScroll = () => {
    y = window.scrollY;
  };

  window.addEventListener("scroll", handleScroll);

  onDestroy(() => {
    window.removeEventListener("scroll", handleScroll);
  });
</script>

<style>
.background {
  background-image: url('/background.jpg');
  background-position: center calc(50% + var(--y) * 0.5px);
  height: 100%;
  width: 100%;
  margin: 0;
  padding: 0;
}
html, body {
  margin: 0;
  padding: 0;
  width: 100%;
  height: 100%;
}
.navbar-link {
  display: inline-block;
  padding: 10px 20px;
  margin: 10px;
  background-color: #5294E2;  /* Changed to Bootstrap's primary color */
  color: #ffffff !important;             /* Changed to white */
  text-decoration: none;
  border-radius: 5px;
  transition: background-color 0.3s ease-in-out;
}

.navbar-link:hover {
  background-color: #0056b3;  /* Darkened version of primary color */
}

.navbar {
  display: flex;
  justify-content:space-around;
  padding: 10px 0;
  width: 100%;
  height:max-content;
  background-color: #333333;  /* Changed to Bootstrap's light color */
  border-radius: 0px; /* New property */
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  margin: 0;
  padding: 0;
}

.navbar-link :global(a){
  color: #000000;  /* Changed to white */
  text-decoration: none;
  display: block;
  width: 100%;
  height: 100%;
}
.navbar-link :global(a:hover), .navbar-link :global(a:focus) {
  color: #000000;
}
</style>


<body>

<div class="background" style="--y: {y}px;">
<Router>
  <nav class="navbar">
    <div class="navbar-link"><Link to="/">Home</Link></div>
    <div class="navbar-link"><Link to="/blog">Blog</Link></div>
    <div class="navbar-link"><Link to="/projects">Projects</Link></div>
  </nav>
  <Route path="/" component={Home} />
  <Route path="/blog" component={Blog} />
  <Route path="/projects" component={Projects} />
  <Route path="/post/:id" let:params={params} component={Post} />
  <Route path="/project/:id" let:params>
    <Project id="{params.id}"/>
  </Route>
  <Route path="/skills/:name" let:params>
    <Skill name="{params.name}"/>
  </Route>
</Router>
</div>
</body>