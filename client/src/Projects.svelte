<!-- Projects.svelte -->
<script>
  import { onMount } from "svelte";
  import { flip } from "svelte/animate";
  import { Link } from "svelte-routing";

  let projects = [
    {
      title: "Audio Visualizer",
      id: 8,
      description:
        "Experience your music like never before with Bevy Audioviz! \
      Crafted with Rust and powered by the Bevy game engine and CPAL library, \
      this real-time audio visualizer captures and processes audio data to display stunning visualizations. \
      Choose from bar, string, or circle visualizers, and tweak your visual experience with an easy-to-use GUI that allows for adjustments in color, \
      frequency range, and more. Enjoy the ultra-low latency and detailed spectrum analysis for a truly immersive audiovisual journey.",
    },
    {
      title: "Fractal",
      id: 7,
      description:
        "Explore the Mandelbrot, Burning Ship, and Julia fractals with unparalleled clarity using the Mandelbrot Fractal Viewer! \
      Engineered with Rust and Bevy, this application delivers ultra-smooth performance and customizable settings for a personalized viewing experience. \
      Toggle effortlessly between fractal types, dive into high-resolution visuals, and manipulate dynamic coloring.",
    },
    {
      title: "Path Finding",
      id: 6,
      description:
        "Explore the intricacies of pathfinding algorithms in real-time! Design your own mazes, choose from A*, Dijkstra's, or BFS, \
      and watch as they find their way.",
    },
    {
      title: "This Portfolio",
      id: 5,
      description:
        "A personal portfolio website engineered using a modern full-stack development framework. \
      The front-end is designed with Svelte, a lightweight, efficient, and reactive JavaScript framework. \
      The back-end is powered by Actix, a Rust-based web framework known for its performance and type safety. \
      This portfolio website showcases various projects, skills, and professional experiences, making it an interactive resume for prospective employers.",
    },
    {
      title: "GPTFit",
      id: 4,
      description:
        "A web application developed to simplify and personalize the creation of exercise programs. \
      By harnessing the power of OpenAI's API, GPTFit generates workout plans tailored to users' fitness levels, goals, and preferences. \
      The integration of AI into the fitness domain exemplifies the potential of machine learning in personalizing digital experiences.",
    },
    {
      title: "Quiz Generation",
      id: 3,
      description:
        "An extensive toolkit for generating and evaluating quizzes from academic content using OpenAI's API. \
      This tool enables educators to automate quiz creation, enhancing efficiency and diversity of questions. \
      Evaluating capability ensures the quality and relevance of generated questions. This project underlines the power of AI in improving the educational sector.",
    },
    {
      title: "Simple RSA",
      id: 2,
      description:
        "The project is an optimized RSA implementation, focusing on rapid 2048-bit prime number generation for efficient key creation. This advancement enhances performance without sacrificing encryption strength, showcasing deep knowledge of RSA protocols and prime number theory.",
    },
    {
      title: "MQTT CMD Internal Testing and Validation",
      id: 1,
      description:
        "A high-performance asynchronous program developed in Rust, \
      designed to interface with an embedded system over MQTT (Message Queuing Telemetry Transport). \
      This system serves as a communication bridge, allowing commands and data to be relayed between the program and the embedded system. \
      It has applications in real-time testing, validation, and troubleshooting of the connected system.",
    },
    {
      title: "2048 Game",
      id: 0,
      description:
        "An interactive recreation of the popular 2048 game. \
      This project is supplemented with an artificial intelligence (AI) agent based on Q-learning, a type of reinforcement learning algorithm. \
      The AI agent autonomously plays the game, making decisions by estimating the future rewards of its actions.",
    },
  ];

  // Function to generate the project page URL
  function getProjectURL(projectId) {
    return `/projects/${projectId}`;
  }

  onMount(async () => {
    //try {
    //  const res1 = await fetch('/api/getProjects');
    //  projects = await res1.json();
    //} catch (error) {
    //  console.error('Error:', error);
    //}
  });
</script>

<!-- Projects Section -->
<div class="projects">
  {#each projects as project (project.id)}
    <div class="project" animate:flip={{ duration: 500 }}>
      <!-- Link to individual project page using Link component -->
      <Link to="/project/{project.title.replaceAll(' ', '_')}">
        <h2>{project.title}</h2>
        <p>{project.description}</p>
      </Link>
    </div>
  {/each}
</div>

<style>
  /* Projects Section */
  .projects {
    display: flex;
    flex-wrap: wrap;
    justify-self: center;
    width: 90%;
    max-width: 800px;
    margin: 1rem auto;
  }

  .project {
    border: 1px solid var(--text-color); /* Using text color for border */
    border-radius: var(--border-radius);
    margin-bottom: 1.0em;
    padding: 1.0em;
    transition: transform 0.3s ease-in-out, box-shadow 0.3s ease-in-out;
    color: var(--text-color); /* Using global text color */
    background-color: var(--secondary-color);
    text-align: center;
  }

  .project:hover {
    transform: scale(1.05);
    box-shadow: 0 10px 20px rgba(0, 0, 0, 0.19), 0 6px 6px rgba(0, 0, 0, 0.23);
  }

  /* Project title style */
  .project h2 {
    color: #e8e8e8;
    margin-bottom: 0.5rem;
  }

  /* Project description style */
  .project p {
    color: #e8e8e8;
    font-size: 1rem;
    line-height: 1.5;
  }

  /* Optional: Change the link text color on hover */
  h1 {
    color: #e8e8e8;
    justify-content: center;
  }
</style>
