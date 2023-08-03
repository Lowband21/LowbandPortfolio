<script>
  import { onMount } from "svelte";
  import { fly } from 'svelte/transition';
  import { flip } from 'svelte/animate';
  import { Link } from 'svelte-routing';

  let name = "Lowband";
  let projects = [];
  let skills = [];
  let bio = "";

  // Your elevator pitch details
  let uniqueTrait = "innovative problem solver";
  let expertise = "systems design and optimization";

  // Function to generate the project page URL
  function getSkillURL(skillId) {
    return `/skills/${skillId}`;
  }

  onMount(async () => {
    try {
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
    padding: 0em;
    background-color: #474747;
    min-height: 100vh;
    transition: all 0.1s ease-in-out;
    position: relative;
    z-index: 1;
  }

  h1, h2 {
    font-size: 3em;
    margin-bottom: 1em;
    color: #333;
  }

  p {
    font-size: 1.5em;
    color: #333;
  }

  .skills {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    padding: 2em;
  }

  .skill {
    border: 1px solid #ddd;
    border-radius: 5px;
    margin: .5em;
    padding: .5em;
    transition: all 0.3s ease-in-out;
    color: #333;
    background-color: #fff;
  }

  .skill:hover {
    transform: scale(1.1);
    box-shadow: 0 10px 20px rgba(0, 0, 0, 0.19), 0 6px 6px rgba(0, 0, 0, 0.23);
  }

  .full-width-container {
    width: 100vw;
    display: flex;
    justify-content: center;
  }

  .welcome-container {
    background-color: #EBF5FB;
    text-align: center;
    padding: 2em 0;
  }

  .bio-container {
    background-color: #EBF5FB;
  }

  .skills-container {
    background-color: #D6EAF8;
    text-align: center;
  }

  .experience-container, .education-container {
    display: flex;
    justify-content: center;
    padding: 2em;
  }

  .experience-container {
    background-color: #AED6F1;
    text-align: left;
  }

  .education-container {
    background-color: #A9CCE3;
    text-align: right;
  }

  .section-content {
    width: 80%;
  }
  .skills-section-content {
    width: 80%;
  }
</style>

<div class="homepage" in:fly={{ x: 0, y: -200, delay: 500, duration: 500 }}>
  <!-- Welcome Section -->
  <div class="full-width-container welcome-container">
    <div class="section-content">
      <h1>Welcome to Lowband's Portfolio!</h1>
    </div>
  </div>

  <!-- Bio Section -->
  <div class="full-width-container bio-container">
    <div class="bio section-content">
      <p>{bio}</p>
    </div>
  </div>


  <!-- Experience Section -->
  <div class="full-width-container experience-container">
    <div class="section-content">
      <h2>Experience</h2>
  
      <h3>University of Denver</h3>
      <p>
        <strong>Generative AI Research Assistant</strong> (Apr 2023 - Present, Hybrid)<br>
        Project leader and researcher for professor Stephen Hutt<br>
        <strong>Teaching Assistant</strong> (Mar 2023 - Jun 2023, Remote)<br>
        Assisted and graded multiple graduate level data science classes.<br>
        <strong>Algorithms and Data Structures Tutor</strong> (Feb 2023 - Mar 2023, Hybrid)<br>
        Tutored students taking Algorithms and Data Structures.<br>
        Skills: Computer Science
      </p>
  
      <h3>Plantern Corp</h3>
      <p>
        <strong>Systems Engineer Intern</strong> (Jul 2022 - Feb 2023, Denver, Colorado)<br>
        Skills: JavaScript, C++, Algorithms, Python, GNU/Linux
      </p>
  
      <h3>Network Solutions, Inc.</h3>
      <p>
        <strong>IT Support Technician Intern</strong> (May 2019 - Oct 2019, Granger, Indiana)<br>
        Gained experience and knowledge working alongside industry professionals.<br>
        Skills: Algorithms
      </p>
    </div>
  </div>

  <!-- Education Section -->
  <div class="full-width-container education-container">
    <div class="section-content">
      <h2>Education</h2>
  
      <h3>University of Denver</h3>
      <p>
        <strong>Bachelor's degree, Computer Science</strong> (Sep 2020 - Jun 2024)<br>
        Current Grade Level: Senior<br>
        Grade in Major: 3.91
      </p>
  
      <h3>Homeschool</h3>
      <p>
        <strong>N/A</strong> (2018 - 2019)<br>
        Grade: 4.19
      </p>
  
      <h3>Penn High School</h3>
      <p>
        <strong>High School Diploma</strong> (2015 - 2018)<br>
        Grade: 4.07
      </p>
    </div>
  </div>

  <!-- Skills Section -->
  <div class="full-width-container skills-container">
    <div class="skills-section-content">
      <div class="skills">
        {#each skills as skill (skill.id)}
          <div class="skill" animate:flip={{ duration: 500 }}>
            <!-- Link to individual skill page using Link component -->
            <Link to={`/skills/${skill.name}`}>
              {skill.name}
            </Link>
          </div>
        {/each}
      </div>
    </div>
  </div>
</div>