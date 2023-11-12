<script lang="ts">
  import Greet from "./lib/Greet.svelte";
  import CallShittyFunc from "./lib/CallShittyFunc.svelte";
  import DragStuffHere from "./lib/DragStuffHere.svelte";
  import toast, { Toaster } from "svelte-french-toast";
</script>

<main class="container">
  <div class="row">
    <Greet />
    <CallShittyFunc />
  </div>
  <DragStuffHere />
</main>
<Toaster />

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>
