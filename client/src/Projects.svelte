<!-- Projects.svelte -->
<script>
  import { onMount } from "svelte";
  import { flip } from "svelte/animate";
  import { Link } from "svelte-routing";

  let projects = [
    {
      title: "Fractal",
      description:
        "A mandelbrot fractal visualizer utilzing rust webassembly and webgl for ultra smooth and detailed rendering.",
    },
    {
      title: "Path Finding",
      description:
        "Explore the intricacies of pathfinding algorithms in real-time! Design your own mazes, choose from A*, Dijkstra's, or BFS, and watch as they find their way. Perfect for learners and professionals alike. Dive in now and discover the magic behind pathfinding!",
    },
    {
      title: "This Portfolio",
      description:
        "A personal portfolio website engineered using a modern full-stack development framework. \
      The front-end is designed with Svelte, a lightweight, efficient, and reactive JavaScript framework. \
      The back-end is powered by Actix, a Rust-based web framework known for its performance and type safety. \
      This portfolio website showcases various projects, skills, and professional experiences, making it an interactive resume for prospective employers.",
    },
    {
      title: "GPTFit",
      description:
        "A web application developed to simplify and personalize the creation of exercise programs. \
      By harnessing the power of OpenAI's API, GPTFit generates workout plans tailored to users' fitness levels, goals, and preferences. \
      The integration of AI into the fitness domain exemplifies the potential of machine learning in personalizing digital experiences.",
    },
    {
      title: "Quiz Generation",
      description:
        "An extensive toolkit for generating and evaluating quizzes from academic content using OpenAI's API. \
      This tool enables educators to automate quiz creation, enhancing efficiency and diversity of questions. \
      Evaluating capability ensures the quality and relevance of generated questions. This project underlines the power of AI in improving the educational sector.",
    },
    {
      title: "Simple RSA Messager",
      description:
        "The project is a secure messaging system that employs RSA, a public-key cryptography protocol, to safeguard the messages exchanged between a client and a server. \
      A unique aspect of this system is the highly efficient generation of 2048-bit prime numbers, a critical component of the RSA algorithm, achieved in a timeframe of approximately 100 milliseconds. \
      This optimization significantly accelerates the key generation process without compromising the robustness of the encryption, providing a secure yet highly responsive communication environment. \
      The performance enhancement demonstrates a deep understanding of the RSA protocol, prime number theory, and algorithmic optimization, and showcases the application of these principles in real-world network programming.",
    },
    {
      title: "MQTT CMD Internal Testing and Validation",
      description:
        "A high-performance asynchronous program developed in Rust, \
      designed to interface with an embedded system over MQTT (Message Queuing Telemetry Transport). \
      This system serves as a communication bridge, allowing commands and data to be relayed between the program and the embedded system. \
      It has applications in real-time testing, validation, and troubleshooting of the connected system.",
    },
    {
      title: "2048 Game",
      description:
        "An interactive recreation of the popular 2048 game. \
      This project is supplemented with an artificial intelligence (AI) agent based on Q-learning, a type of reinforcement learning algorithm. \
      The AI agent autonomously plays the game, making decisions by estimating the future rewards of its actions. \
      This project demonstrates proficiency in game development and machine learning.",
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
    width: 90%;
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
