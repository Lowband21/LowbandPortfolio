<script>
  export let post = {
    title: "",
  };
</script>

<style>
  .post {
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
</style>

<div class='post'>
  <h2>{post.title}</h2>
  <section>
    <h1>I Take it Back: Actix > Flask</h1>
    <p>
      My journey of web development started with the enriching experience of
      Svelte and Flask, offering a delightful blend of performance and
      simplicity. Over time, I've gained a substantial amount of experience and
      expertise in this tech stack. However, the endless thirst for knowledge
      and constant curiosity led me to explore Rust and its Actix web framework.
    </p>
    <p>
      This transition from Flask, a Python-based microframework, to Rust Actix,
      was driven by my ambition to utilize the robustness, efficiency, and type
      safety that Rust offers. Not only am I now proficient in Rust Actix, but
      I'm also able to say that my skills in Rust are almost on par with Python.
      In this blog post, I'll detail my transition journey, the challenges I
      faced, and the lessons I learned.
    </p>
  </section>
  <section>
    <h2>The Allure of Rust Actix</h2>
    <p>
      Rust is a systems programming language, known for its speed, memory
      safety, and parallelism. It's particularly effective for web development
      when coupled with Actix, an actor system and web framework offering high
      performance and a rich set of tools. Unlike Flask, where ease of use
      occasionally compromises execution speed, Rust offers both power and
      performance due to its low-level capabilities and zero-cost abstractions.
      Actix takes advantage of these Rust features, providing a flexible and
      high-performance web framework.
    </p>
  </section>
  <section>
    <h2>My Journey from Flask to Rust Actix</h2>
    <h3>Setting Up</h3>
    <p>
      Transitioning from Flask to Rust Actix posed a steep learning curve,
      primarily due to the paradigm shift from Python to Rust. However, Rust’s
      package manager Cargo made the setup process as smooth as Flask's pipenv
      or venv.
    </p>
    <h3>Routing</h3>
    <p>
      Flask handles routes with decorators, an approach that's simple and
      intuitive. Actix, on the other hand, uses a function-based approach where
      routes are defined during the application service configuration. This
      method felt verbose initially but turned out to offer more flexibility and
      power.
    </p>
    <h3>Request and Response Handling</h3>
    <p>
      Actix leverages Rust's powerful type system and async functionality for
      request and response handling. This was a major change from Flask, which
      uses dynamically typed functions for similar tasks.
    </p>
    <h3>Database Connectivity</h3>
    <p>
      While Flask leverages SQLAlchemy for communication with relational
      databases, Rust Actix uses Diesel, a type-safe ORM and query builder. This
      posed a significant learning curve, but I enjoyed the deep understanding
      of models and migrations that it fostered.
    </p>
    <h3>Error Handling</h3>
    <p>
      Rust's strict error handling rules, enforced by its powerful type system,
      were quite different from the laid-back approach in Flask. This ensured
      that all possible errors were appropriately handled.
    </p>
  </section>
  <section>
    <h2>Conclusion</h2>
    <p>
      Transitioning from Flask to Rust Actix was not only a stepping stone into
      the Rust world, but it also served as a challenging and rewarding
      experience. Rust Actix provides performance, robustness, and type safety,
      posing a stark contrast to Flask's simplicity and flexibility.
    </p>
    <p>
      While Rust Actix indeed has a steep learning curve, the benefits reaped in
      terms of performance make it a compelling framework for web development.
      As I continue to delve deeper into Rust, I look forward to sharing more
      insights from my journey.
    </p>
  </section>
</div>
