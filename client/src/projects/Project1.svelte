<!-- Post1.svelte -->
<script>
  import Particles from "svelte-particles";
  let particlesConfig = {
    fpsLimit: 60,
    particles: {
      color: {
        value: "#000"
      },
      links: {
        enable: true,
        color: "#000"
      },
      move: {
        enable: true
      }
    }
  };
</script>

<style>
  :global(#tsparticles) {
    margin: 0;
    padding: 0;
    position: fixed;
    width: 100%;
    height: 100%;
    top: 0;
    left: 0;
    z-index: -1;
  }
</style>
  
<div>
  <h1>A Title<h1>
</div>
<Particles options={particlesConfig} />