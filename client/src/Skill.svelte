<script>
  import { onMount } from "svelte";
  import Markdown from "./skills/Markdown.svelte";
  import Modal_Editing from "./skills/Modal_Editing.svelte";
  import Python from "./skills/Python.svelte";

  export let name; // this is the project id, which we receive as a prop

  let ProjectComponent;

  onMount(() => {
    console.log("Skill component loaded");
    switch (name) {
      case 'Modal_Editing':
        ProjectComponent = Modal_Editing;
        break;
      case 'Python':
        ProjectComponent = Python;
        break;
      case 'Markdown':
        ProjectComponent = Markdown;
        break;
      // handle more cases as needed
      default:
        console.error(`Unknown project ID: ${id}`);
        ProjectComponent = null;
    }
  });
</script>

<svelte:component this={ProjectComponent} />