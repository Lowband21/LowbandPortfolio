import Post1 from './Post1.svelte';
import Post2 from './Post2.svelte';

export const blogPosts = [
  {
    component: Post1,
    metadata: {
        title: "Unleashing the Power of Full Stack Development with Svelte and Flask",
        brief: "This website was written with a combination of Svelte and Flask, this this how it went."
    }
  },
  {
    component: Post2,
    metadata: {
        title: "I take it back: Actix > Flask",
        brief: `Venture with me as I navigate my journey from Python's Flask to Rust's Actix. Experience the challenges and victories encountered as I master Rust's robust, efficient, and type-safe traits. This personal account will highlight my transition, focusing on setting up, routing, handling requests and responses, database connectivity, and error handling in Rust Actix. Despite a steep learning curve, I found Rust Actix's performance and robustness incredibly rewarding, enhancing my web development repertoire.`,
    }
  }
  // Add more posts as needed
];

export default blogPosts;
