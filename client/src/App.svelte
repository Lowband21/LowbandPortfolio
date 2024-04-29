<script>
  import { Router, Route, Link } from "svelte-routing";
  import Home from "./Home.svelte";
  import Blog from "./Blog.svelte";
  import Projects from "./Projects.svelte";
  import Chat from "./Chat.svelte";
  import Post from "./Post.svelte";
  import Project from "./Project.svelte"; // New line
  import Skill from "./Skill.svelte"; // New line
  import "./global.css";

  import Particles from "svelte-particles";

  import Typed from 'typed.js';

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
        value: 200,
      },
    },
  };
  let navbarExpanded = false;

  function expandNavbar(){
    setTimeout(() => {
      // Add the class after the width transition duration
      document.querySelectorAll('.link-text').forEach(link => {
        link.classList.remove('link-text');
        link.classList.add('link-text-visible');
      });
    }, 500); // Duration of the navbar's width transition
  }
  function collapseNavbar() {
    // Remove the class immediately when mouse leaves
    document.querySelectorAll('.link-text-visible').forEach(link => {
      link.classList.remove('link-text-visible');
      link.classList.add('link-text');
    });
  }

  function handleNavbarHover() {
    navbarExpanded = !navbarExpanded;

    if (navbarExpanded) {
      setTimeout(() => {
        // Add the class after the width transition duration
        document.querySelectorAll('.link-text').forEach(link => {
          //link.classList.remove('link-text');
          link.classList.add('link-text-visible');
        });
      }, 500); // Duration of the navbar's width transition
    } else {
      // Remove the class immediately when mouse leaves
      document.querySelectorAll('.link-text').forEach(link => {
        link.classList.remove('link-text-visible');
        //link.classList.add('link-text');
      });
    }
  }
</script>

<Particles options={particlesConfig} />
<div class="background">
  <Router>
    <nav class="navbar" on:mouseenter={expandNavbar} on:mouseleave={collapseNavbar}>
      <div class="link-container">
        <Link to="/" class="navbar-link">
          <i class="fas fa-home"></i>
          <span class="link-text">Home</span>
        </Link>
      </div>
      <div class="link-container">
      <Link to="/blog" class="navbar-link">
        <i class="fas fa-blog"></i>
        <span class="link-text">Blog</span>
      </Link>
      </div>
      <div class="link-container">
      <Link to="/chat" class="navbar-link">
        <i class="fas fa-comments"></i>
        <span class="link-text">Chat</span>
      </Link>
      </div>
      <div class="link-container">
      <Link to="/projects" class="navbar-link">
        <i class="fas fa-project-diagram"></i>
        <span class="link-text">Projects</span>
      </Link>
      </div>
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

  .navbar {
    position: fixed; /* Fixed position */
    top: 0;
    left: 0;
    height: 100vh; /* Full viewport height */
    width: var(--navbar-width-collapsed); /* Adjust this as needed */
    display: flex;
    flex-direction: column; /* Align links vertically */
    align-items: center; /* Center links horizontally */
    background-color: var(--secondary-color);
    box-shadow: var(--box-shadow);
    z-index: 9999;
    transition: width 0.5s ease-in-out; /* Transition for width */
  }
  .navbar:hover {
    width: var(--navbar-width-expanded); /* Width when expanded */
    transition: width 0.5s ease-in-out; /* Transition for width */
  }

  .navbar-link :global(a) {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: flex-start;
    width: 100%; /* Full width of the container */
    height: 100%; /* Full height of the container */
    color: var(--text-color);
    text-decoration: none;
    padding: var(--navbar-padding); /* Adjust vertical padding */
    border: 1px solid #ccc; /* Example border, change as needed */
  }

  .link-container {
    display: flex;
    align-items: center; /* Vertically center the contents */
    justify-content: flex-start; /* Align items to the start */
    color: var(--text-color);
    padding: var(--navbar-padding);
    width: 100%;
    height: 80px;
  }

  .navbar-link {
    display: flex;
    align-items: center;
    justify-content: flex-start;
    width: 100%; /* Full width of the container */
    text-decoration: none;
  }

  .fas, .link-text {
    vertical-align: middle; /* Align inline elements to the middle */
  }

  .fas {
    font-size: 2.0em; /* Adjust icon size */
    margin-right: 10px; /* Space between icon and text */
  }

  .link-text {
    font-size: 2.0em; /* Adjust text size */
    display: inline-block;
    opacity: 0;
    width: 0;
    overflow: hidden;
    white-space: nowrap;
    transition: opacity 4.5s, width 3.5s;
  }

  .link-text-visible {
    opacity: 1;
    width: auto;
    transition: opacity 5s, width 5s;
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
