<!-- Post1.svelte -->
<script>
  console.log("Post1 Loaded");
  let title = "Building My Portfolio Website";
  let date = "September 29, 2023";
  let author = "Grayson Hieb";
</script>

<style>
  .blog-post {
    width: 90%;
    margin: auto;
    background-color: #333333;
    margin-top: 20px;
    padding-left: 10px;
    padding-right: 10px;
    border-radius: 5px;
    border: 3px solid #000;
  }
  h1, h2, h3, p {
    color: #e8e8e8;
  }

  header {
    margin-bottom: 2rem;
  }

  h1 {
    font-size: 2.5rem;
    margin-bottom: 0.5rem;
  }

  p {
    margin-bottom: 1.5rem;
  }
</style>

<article class="blog-post">
 <header>
   <h1>{title}</h1>
   <p>Published on {date} by {author}</p>
 </header>

 <div class="content">
   <p>When I set out to create my portfolio website, I knew I wanted to use a modern, innovative tech stack that would not only showcase my skills but also be a joy to work with. Rust, with its performance, safety, and delightful developer experience, was an obvious choice for me. I absolutely love working with Rust, and I couldn't pass up the opportunity to use it for this project.</p>

   <p>For the web framework, I chose Actix. It's stable, well-supported, and has excellent documentation. Plus, Actix is built on top of Rust's powerful async/await primitives, which allows for efficient handling of concurrent requests. It also integrates seamlessly with other Rust libraries and frameworks, making it a solid choice for building web applications.</p>

   <p>On the frontend, I decided to go with Svelte. I was drawn to its novel and easy-to-grasp component system, which makes building interactive user interfaces a breeze. Svelte's popularity is on the rise, and I wanted to explore its capabilities firsthand.</p>

   <p>To be honest, my journey wasn't a straight path. I initially started with Flask for the backend and later switched to Actix. I also experimented with Diesel for database interactions before settling on SQLx. It was a process of trial and error, following guides and documentation to determine best practices. Throughout the development, I focused on correctness, performance, and refactor-ability.</p>

   <p>Structuring the project was an interesting challenge. The backend, written in Rust, serves as the main project folder, while the Svelte frontend resides in a subfolder. I organized the backend routes by type and the frontend components by their purpose, with separate folders for blog posts and projects. The frontend compiles the static files, which are then served by the backend.</p>

   <p>I did encounter some struggles along the way, particularly with database interactions using Diesel and SQLx. While not inherently difficult, ensuring that everything aligned correctly was a tedious process. I also faced some issues with dependencies and rollup configuration on the frontend. In contrast, the Rust backend always worked flawlessly, further reinforcing my appreciation for Rust and my growing dislike for dynamically typed and interpreted languages.</p>

   <p>For data persistence, I incorporated a Redis server and a PostgreSQL database, although the majority of the website's content is static and doesn't heavily rely on them. Nonetheless, I'm glad I gained experience working with these technologies, and I'm sure I'll find opportunities to utilize them more extensively in future enhancements.</p>

   <p>Looking back, if I were to start over, I would have begun with Rust, Actix, SQLx, and Svelte right from the start. I would have invested more time in checking dependencies and recommended versions, and spent more effort planning the project from the beginning. However, I'm quite satisfied with how the trial-and-error approach turned out in the end.</p>

   <p>I'm particularly proud of the website's home page and the interactive projects I built. The design looks aesthetic, and the layout is highly functional. This project not only allowed me to explore new technologies but also challenged me to think critically and solve problems creatively.</p>

   <p>In the end, building my portfolio website was an incredible learning experience. It gave me the opportunity to work with cutting-edge technologies, showcase my skills, and create something I'm truly proud of. I look forward to continuing to refine and expand my website, and I'm excited to see where this journey takes me next.</p>
 </div>
</article>
