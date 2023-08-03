<!-- Projects.svelte -->
<script>
  import { onMount } from "svelte";
  import { flip } from 'svelte/animate';

  let projects = [];

  // Function to generate the project page URL
  function getProjectURL(projectId) {
    return `/projects/${projectId}`;
  }

  onMount(async () => {
    try {
      const res1 = await fetch('/api/getProjects');
      projects = await res1.json();
    } catch (error) {
      console.error('Error:', error);
    }
  });
</script>

<style>
  /* Projects Section */
  .projects {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    padding: 1em;
    background-color: #D6EAF8;
  }

  .project {
    border: 1px solid #ddd;
    border-radius: 5px;
    margin: .5em;
    padding: .5em;
    transition: all 0.3s ease-in-out;
    color: #333;
    background-color: #fff;
  }

  .project:hover {
    transform: scale(1.1);
    box-shadow: 0 10px 20px rgba(0, 0, 0, 0.19), 0 6px 6px rgba(0, 0, 0, 0.23);
  }

  /* Project title style */
  .project h3 {
    color: #333;
    font-size: 1.5rem;
    margin-bottom: 0.5rem;
  }

  /* Project description style */
  .project p {
    color: #333;
    font-size: 1rem;
    line-height: 1.5;
  }

  /* Optional: Change the link text color on hover */
  .project h3:hover, .project p:hover {
    color: #007bff;
  }
</style>

<!-- Projects Section -->
<div class="projects">
  {#each projects as project (project.id)}
    <div class="project" animate:flip={{ duration: 500 }}>
      <!-- Link to individual project page using anchor tag -->
      <a href="{getProjectURL(project.id)}">
        <h3>{project.name}</h3>
        <p>{project.description}</p>
      </a>
    </div>
  {/each}
</div>