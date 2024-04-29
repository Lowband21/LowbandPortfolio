import Post1 from './Post1.svelte';
import Post2 from './Post2.svelte';
import RustIntegration from './RustIntegration.svelte';

export const blogPosts = [
  {
    component: Post1,
    metadata: {
        title: "Building My Portfolio Website",
        brief: "This website was written with a combination of Svelte and Actix, this this how it went."
    }
  },
  {
    component: RustIntegration,
    metadata: {
        title: "Integrating my Projects",
        brief: `In this post, the author delves into the journey of incorporating their existing Rust projects into this portfolio website. Highlighting Rust's core strengths—memory safety, concurrency, and speed—the article details how the language was used to enhance the performance of mathematically intensive tasks for RSA encryption. Through the use of Actix and Svelte, prime generation was seamlessly integrated into the website, providing near-native performance and showcasing the vast possibilities when merging diverse tech stacks. The author concludes by reflecting on the enlightening experience and the future potential of such integrations in the ever-evolving web technology landscape.`,
    }
  }
];

export default blogPosts;
