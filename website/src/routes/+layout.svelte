<script lang="ts">
  import "../app.css";

  import Header from "$lib/Header.svelte";
  import Footer from "$lib/Footer.svelte";
</script>

<div class="w-full">
  <Header />
  <main class="flex-auto">
    <slot />
  </main>
  <Footer />
</div>

<style lang="postcss">
  :global(html, body) {
    @apply w-full max-w-full overflow-x-hidden;
  }

  main {
    @apply w-full;
  }
</style>
