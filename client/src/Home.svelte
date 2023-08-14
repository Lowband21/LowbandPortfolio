<script>
  import { onMount, onDestroy } from "svelte";
  import { fly } from "svelte/transition";
  import { flip } from "svelte/animate";
  import { Link } from "svelte-routing";
  import Particles from "svelte-particles";

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
      const res2 = await fetch("/api/getSkills");
      skills = await res2.json();

      const res3 = await fetch("/api/getBio");
      bio = await res3.json();
      bio = bio.bio_content;
    } catch (error) {
      console.error("Error:", error);
    }
  });
</script>

<div class="homepage">
  <!-- Welcome Section -->
  <div class="full-width-container welcome-container">
    <div class="section-content">
      <h1>Welcome to Lowband's Portfolio!</h1>
    </div>
  </div>

  <!-- Bio Section -->
  <div class="full-width-container bio-container">
    <div class="bio section-content text-block">
      <h2>About Me</h2>
      <p>{bio}</p>
    </div>
  </div>

  <!-- Experience Section -->
  <div class="full-width-container experience-container">
    <div class="section-content text-block">
      <h2>Experience</h2>

      <h3>University of Denver</h3>
      <p>
        <strong>Generative AI Research Assistant</strong> (Apr 2023 - Present,
        Hybrid)<br />
        Project leader and researcher for professor Stephen Hutt working on <Link
          to={"/project/Quiz_Generation"}>quiz generation</Link
        ><br />
        <strong>Teaching Assistant</strong> (Mar 2023 - Jun 2023, Remote)<br />
        Assisted and graded multiple graduate level data science classes.<br />
        <strong>Algorithms and Data Structures Tutor</strong> (Feb 2023 - Mar
        2023, Hybrid)<br />
        Tutored students taking Algorithms and Data Structures.<br />
        Skills: Computer Science
      </p>

      <h3>Plantern Corp</h3>
      <p>
        <strong>Systems Engineer Intern</strong> (Jul 2022 - Feb 2023, Denver,
        Colorado)<br />
        Skills: JavaScript, C++, Algorithms, Python, GNU/Linux
      </p>

      <h3>Network Solutions, Inc.</h3>
      <p>
        <strong>IT Support Technician Intern</strong> (May 2019 - Oct 2019,
        Granger, Indiana)<br />
        Gained experience and knowledge working alongside industry professionals.<br
        />
        Skills: Algorithms
      </p>
    </div>
  </div>

  <!-- Education Section -->
  <div class="full-width-container education-container">
    <div class="section-content text-block">
      <h2>Education</h2>

      <h3>University of Denver</h3>
      <p>
        <strong>Bachelor's degree, Computer Science</strong> (Sep 2020 - Jun
        2024)<br />
        Current Grade Level: Senior<br />
        Grade in Major: 3.91
      </p>

      <h3>Homeschool</h3>
      <p>
        <strong>N/A</strong> (2018 - 2019)<br />
        Grade: 4.19
      </p>

      <h3>Penn High School</h3>
      <p>
        <strong>High School Diploma</strong> (2015 - 2018)<br />
        Grade: 4.07
      </p>
    </div>
  </div>

  <!-- Skills Section -->
  <div class="full-width-container skills-container">
    <div class="full-width-container skills-section-content">
      <div class="skills">
        {#each skills as skill (skill.id)}
          <div class="skill" animate:flip={{ duration: 500 }}>
            <!-- Link to individual skill page using Link component -->
            <Link to={`/skills/${skill.name.replace(" ", "_")}`}>
              {skill.name}
            </Link>
          </div>
        {/each}
      </div>
    </div>
  </div>
</div>

<style>
  *,
  *::before,
  *::after {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }

  .homepage {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-between;
    padding: 0em;
    min-height: 100vw;
    transition: all 0.1s ease-in-out;
    position: relative;
    z-index: 1;
  }

  .text-block {
    background-color: #333333;
    border: 3px solid #000000;
    padding: 20px; /* Add some padding so that the background extends beyond just the text */
    border-radius: 20px;
    opacity: 90%;
  }

  h1 {
    font-size: 3em;
    color: #000000;
  }

  h2 {
    font-size: 3em;
    margin-bottom: 1em auto;
    color: #ffffff;
    text-justify: center;
    justify-content: center;
    justify-self: center;
  }

  h3 {
    font-size: 2em;
    margin-bottom: 1em;
    color: #ffffff;
  }

  p {
    font-size: 1.5em;
    color: #ffffff;
  }

  .skills {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    padding: 2em;
    background-color: #333333;
  }

  .skill {
    border-radius: 5px;
    margin: 0.5em;
    padding: 0.5em;
    transition: all 0.3s ease-in-out;
    color: #333;
    background-color: #ffffff;
  }

  .skill:hover {
    transform: scale(1.1);
    box-shadow: 0 10px 20px rgba(0, 0, 0, 0.19), 0 6px 6px rgba(0, 0, 0, 0.23);
  }

  .full-width-container {
    width: 100%;
    margin-bottom: 0px;
    padding-bottom: 0px;
    display: flex;
    justify-content: center;
  }

  .welcome-container {
    width: 80%;
    text-align: center;
    padding: 2em 0;
  }

  .bio-container {
    text-align: center;
    width: 100%;
  }

  .skills-container {
    text-align: center;
    align-self: center;
    margin-bottom: 0px;
    padding-bottom: 0px;
    width: 100%;
    border: 3px solid #000000;
  }

  .experience-container,
  .education-container {
    display: flex;
    justify-content: center;
  }

  .experience-container {
    text-align: left;
  }

  .education-container {
    text-align: right;
  }

  .section-content {
    width: 90%;
    margin-bottom: 10px;
  }
  .skills-section-content {
    width: 100%;
    padding-top: 0;
    padding-bottom: 0;
    margin-top: 0;
    margin-bottom: 0;
  }
</style>
