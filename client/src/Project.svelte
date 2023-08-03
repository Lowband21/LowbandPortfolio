<script>
  import { onMount } from "svelte";
  import Project1 from "./projects/Project1.svelte";
  import Project2 from "./projects/Project2.svelte";
  // import more projects as needed

  export let id; // this is the project id, which we receive as a prop

  let ProjectComponent;

  onMount(() => {
    switch (id) {
      case '1':
        ProjectComponent = Project1;
        break;
      case '2':
        ProjectComponent = Project2;
        break;
      // handle more cases as needed
      default:
        console.error(`Unknown project ID: ${id}`);
        ProjectComponent = null;
    }
  });
</script>

<svelte:component this={ProjectComponent} />