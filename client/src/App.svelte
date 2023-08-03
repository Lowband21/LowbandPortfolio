<script>
  import { onMount } from "svelte";
  import { Router, Route, Link } from 'svelte-routing';
  import Home from './Home.svelte';
  import Blog from './Blog.svelte';
  import Projects from './Projects.svelte';
</script>

<style>
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
  justify-content: space-around;
  padding: 10px 0;
  background-color: #404552;  /* Changed to Bootstrap's light color */
  border-radius: 15px; /* New property */
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
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

<Router>
  <nav class="navbar">
    <div class="navbar-link"><Link to="/">Home</Link></div>
    <div class="navbar-link"><Link to="/blog">Blog</Link></div>
    <div class="navbar-link"><Link to="/projects">Projects</Link></div>
  </nav>
  <Route path="/" component={Home} />
  <Route path="/blog" component={Blog} />
  <Route path="/projects" component={Projects} />
</Router>