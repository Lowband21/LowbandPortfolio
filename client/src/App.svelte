<script>
  import { onMount, onDestroy } from "svelte";
  import { Router, Route, Link } from "svelte-routing";
  import Home from "./Home.svelte";
  import Blog from "./Blog.svelte";
  import Projects from "./Projects.svelte";
  import Chat from "./Chat.svelte";
  import Post from "./Post.svelte";
  import Project from "./Project.svelte"; // New line
  import Skill from "./Skill.svelte"; // New line

  import Particles from "svelte-particles";

  let particlesConfig = {
    fpsLimit: 120,
    particles: {
      color: {
        value: "#000",
      },
      links: {
        enable: true,
        color: "#000",
      },
      move: {
        enable: true,
      },
      number: {
        value: 500,
      },
    },
  };
</script>

<Particles options={particlesConfig} />
<div class="background">
  <Router>
    <nav class="navbar">
      <div class="navbar-link"><Link to="/">Home</Link></div>
      <div class="navbar-link"><Link to="/blog">Blog</Link></div>
      <div class="navbar-link"><Link to="/chat">Chat</Link></div>
      <div class="navbar-link"><Link to="/projects">Projects</Link></div>
    </nav>
    <Route path="/" component={Home} />
    <Route path="/blog" component={Blog} />
    <Route path="/chat" component={Chat} />
    <Route path="/projects" component={Projects} />
    <Route path="/post/:id" let:params component={Post} />
    <Route path="/project/:id" let:params>
      <Project id={params.id} />
    </Route>
    <Route path="/skills/:name" let:params>
      <Skill name={params.name} />
    </Route>
  </Router>
</div>

<style>
  .background {
    height: 100%;
    width: 100%;
    margin: 0;
    padding: 0;
  }
  html,
  body {
    margin: 0;
    padding: 0;
    width: 100%;
    height: 100%;
  }
  .navbar-link {
    display: inline-block;
    padding: 10px 20px;
    margin: 10px;
    background-color: #5294e2; /* Changed to Bootstrap's primary color */
    color: #ffffff !important; /* Changed to white */
    text-decoration: none;
    border-radius: 5px;
    transition: background-color 0.3s ease-in-out;
  }

  .navbar-link:hover {
    background-color: #0056b3; /* Darkened version of primary color */
  }

  .navbar {
    display: flex;
    justify-content: space-around;
    padding: 10px 0;
    width: 100%;
    height: max-content;
    background-color: #333333; /* Changed to Bootstrap's light color */
    border-radius: 0px; /* New property */
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    margin: 0;
    padding: 0;
    z-index: 9999;
  }

  .navbar-link :global(a) {
    color: #000000; /* Changed to white */
    text-decoration: none;
    display: block;
    width: 100%;
    height: 100%;
  }
  .navbar-link :global(a:hover),
  .navbar-link :global(a:focus) {
    color: #000000;
  }
  :global(#tsparticles) {
    margin: 0;
    padding: 0;
    position: fixed;
    width: 100%;
    height: 100%;
    top: 0;
    left: 0;
    z-index: -9999;
  }
</style>

