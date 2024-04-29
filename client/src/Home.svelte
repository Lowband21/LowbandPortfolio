<script>
  import { onMount, onDestroy } from "svelte";
  import { fly } from "svelte/transition";
  import { flip } from "svelte/animate";
  import { Link } from "svelte-routing";

  let name = "Lowband";
  let skills = [];
  let bio = "I'm a meticulous and driven Computer Science student from the University of Denver, boasting a robust academic performance. My toolkit of programming languages spans Python, Rust, C&C++, Javascript, Haskell and Java, with a particular focus on Rust and Python, owing to my extensive practical experience. Beyond programming, I also bring proficiency in Linux system administration, Docker, Virtualization, and Shell Scripting. My passion lies in leveraging these skills to solve complex problems and innovate.";

  // Function to generate the project page URL
  function getSkillURL(skillId) {
    return `/skills/${skillId}`;
  }
  let scrolled = false;

  function handleScroll() {
    if (window.scrollY > 50 && !scrolled) {
      scrolled = true;
    } else if (window.scrollY <= 50 && scrolled) {
      scrolled = false;
    }
  }

  function scrollToContent() {
    const contentSection = document.querySelector('.homepage');
    contentSection.scrollIntoView({ behavior: 'smooth' });
  }

  onMount(() => {
    window.addEventListener('scroll', handleScroll);
  });

  onDestroy(() => {
    window.removeEventListener('scroll', handleScroll);
  });
  onMount(async () => {
    try {
      const res2 = await fetch("/api/getSkills");
      skills = await res2.json();

      //const res3 = await fetch("/api/getBio");
      //bio = await res3.json();
      //bio = bio.bio_content;
    } catch (error) {
      console.error("Error:", error);
    }
  });
</script>
<div class:scrolled="{scrolled}" class="landing-section">
  <div class="welcome-container">
    <h1>Welcome to Lowband's Portfolio!</h1>
    <button on:click="{scrollToContent}" id="scroll_button">
      <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" fill="currentColor" class="bi bi-arrow-down-circle" viewBox="0 0 16 16">
        <path fill-rule="evenodd" d="M1 8a7 7 0 1 0 14 0A7 7 0 0 0 1 8zm15 0A8 8 0 1 1 0 8a8 8 0 0 1 16 0zM8.5 4.5a.5.5 0 0 0-1 0v5.793L5.354 8.146a.5.5 0 1 0-.708.708l3 3a.5.5 0 0 0 .708 0l3-3a.5.5 0 0 0-.708-.708L8.5 10.293V4.5z"></path>
      </svg>
    </button>
  </div>
</div>
<div class="homepage">
  <!-- Bio Section -->
  <div class="bio-container">
    <div class="text-block">
      <h2>About Me</h2>
      <p>{bio}</p>
    </div>
  </div>

  <div class="experience-container">
    <div class="text-block">
      <h2>Experience</h2>

      <h3>University of Denver</h3>
      <p>
        <strong>Generative AI Research Assistant</strong> (Apr 2023 - Present,
        Hybrid)<br />
        Research assistant for professor Stephen Hutt working on <Link
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
  <div class="education-container">
    <div class="text-block">
      <h2>Education</h2>

      <h3>University of Denver</h3>
      <p>
        <strong>Bachelor's degree, Computer Science</strong> (Sep 2020 - Jun
        2024)<br />
        Current Grade Level: Senior<br />
        Grade in Major: 3.98
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
  <div class="skills-container">
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

<style>
  .landing-section.scrolled {
    transform: translateY(-100vh); /* Move the landing section up */
  }

  .homepage.scrolled {
    margin-top: 0; /* Reset margin to show the content */
  }
  .landing-section {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    height: 100vh; /* Full viewport height */
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1;
    transition: transform 0.5s ease-in-out;
  }

  :global(body) {
    overflow-x: hidden; /* Prevent horizontal scrolling */
    margin: 0;
    padding: 0;
    box-sizing: border-box;
    font-family: var(--font-primary);
    background-color: var(--background-color);
    color: var(--text-color);
    line-height: 1.6;
  }

  .homepage {
    margin-top: 100vh; /* Push content below the viewport */
    transition: margin-top 0.5s ease-in-out;
    z-index: -5;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: flex-start;
    padding-top: 5vh;
  }

  .welcome-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
  }

  #scroll_button {
    position: absolute;
    bottom: 2rem;
    left: 50%;
    transform: translateX(-50%);
    background: none;
    border: none;
    color: var(--primary-color);
    cursor: pointer;
    transition: transform 0.3s ease;
  }

  #scroll_button:hover {
    transform: translateX(-50%) scale(1.1);
  }

  .welcome-container, .bio-container, .experience-container, .education-container {
    text-align: center;
    max-width: 800px;
    width: 90%;
    margin: 2rem auto;
  }

  h1, h2, h3 {
    font-weight: 300;
  }

  h1 {
    color: var(--primary-color);
  }

  h2 {
    color: var(--primary-color);
    margin-top: 1rem;
  }

  .text-block {
    background-color: var(--secondary-color);
    padding: 2rem;
    margin-bottom: 2rem;
    border-radius: 10px;
    box-shadow: 0 5px 15px rgba(0,0,0,0.1);
  }

  .skills-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    margin-bottom: 5vh;
    padding-left: 2rem; /* Add left padding */
    padding-right: 2rem; /* Add right padding */
  }

  .skills {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    gap: 1rem;
  }

  .skill {
    background-color: #2C3A47;
    color: white;
    padding: 0.5rem 1rem;
    border-radius: 5px;
    transition: transform 0.3s ease;
    cursor: pointer;
  }

  .skill:hover {
    transform: translateY(-5px);
    box-shadow: 0 10px 20px rgba(0,0,0,0.2);
  }

  a {
    color: white;
    text-decoration: none;
  }
</style>
