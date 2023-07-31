<script>
  import { onMount } from "svelte";
  import { fly } from 'svelte/transition';
  import { flip } from 'svelte/animate';
  let name = "Lowband";
  let projects = [];
  let skills = [];
  let bio = "";

  onMount(async () => {
    try {
      const res1 = await fetch('/api/getProjects');
      projects = await res1.json();

      const res2 = await fetch('/api/getSkills');
      skills = await res2.json();

      const res3 = await fetch('/api/getBio');
      bio = await res3.text();
    } catch (error) {
      console.error('Error:', error);
    }
  });
</script>

<style>
  .homepage {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 2em;
    background-image: url('/background.png');
    background-position: center;
    background-repeat: no-repeat;
    background-size: cover;
    color: #fff;
    min-height: 100vh;
    transition: all 0.5s ease-in-out;
  }

  h1, h2 {
    font-size: 2em;
    margin-bottom: 1em;
  }

  .skills, .projects {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    padding: 1em;
  }

  .skill, .project {
    border: 1px solid #ddd;
    border-radius: 5px;
    margin: .5em;
    padding: .5em;
    background: white;
    transition: all 0.3s ease-in-out;
    color: #3c4043; /* Add this line */
  }

  .skill:hover, .project:hover {
    transform: scale(1.1);
    box-shadow: 0 10px 20px rgba(0, 0, 0, 0.19), 0 6px 6px rgba(0, 0, 0, 0.23);
  }
</style>

<div class="homepage" in:fly={{ x: 0, y: -200, delay: 500, duration: 500 }}>
  <h1>Welcome to {name}'s Portfolio</h1>
  <p>{bio}</p>

  <h2>Skills</h2>
  <div class="skills">
    {#each skills as skill (skill.id)}
      <div class="skill" animate:flip={{ duration: 500 }}>{skill.name}</div>
    {/each}
  </div>

  <h2>Projects</h2>
  <div class="projects">
    {#each projects as project (project.id)}
      <div class="project" animate:flip={{ duration: 500 }}>
        <h3>{project.name}</h3>
        <p>{project.description}</p>
      </div>
    {/each}
  </div>
</div>
