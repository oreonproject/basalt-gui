<script>
  let { children } = $props();
  import "../app.css";
  import Titlebar from "$lib/Titlebar.svelte";
</script>

<Titlebar />
<main class="mt-8 h-[calc(100vh-2rem)] overflow-auto">
  {@render children()}
</main>