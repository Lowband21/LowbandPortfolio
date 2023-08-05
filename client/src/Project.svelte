<script>
  import { onMount } from "svelte";
  import Project1 from "./projects/Project1.svelte";
  import Project2 from "./projects/Project2.svelte";
  import Quiz_Gen from "./projects/Quiz_Gen.svelte";
  import RSA_Messager from "./projects/RSA_Messager.svelte";
  // import more projects as needed

  export let id; // this is the project id, which we receive as a prop

  let ProjectComponent;

  onMount(() => {
    switch (id) {
      case 'This_Portfolio':
        ProjectComponent = Project1;
        break;
      case 'GPTFit':
        ProjectComponent = Project2;
      case 'Quiz_Generation':
        ProjectComponent = Quiz_Gen;
      case 'Simple_RSA_Messager':
        ProjectComponent = RSA_Messager;
        break;
      // handle more cases as needed
      default:
        console.error(`Unknown project ID: ${id}`);
        ProjectComponent = null;
    }
  });
</script>

<style>

  .text-block {
    width: 90%;
    margin: auto;
    background-color: #333333;
    margin-top: 20px;
    color:white;
    padding-right: 10px;
    padding-left: 10px;
    padding-top: 5px;
    padding-bottom: 5px;
    margin-bottom: 20px;
    border-radius: 5px;
    justify-self: center;
    border: 3px solid #000;
  }
</style>

<div class=text-block>
  <svelte:component this={ProjectComponent} />
</div>