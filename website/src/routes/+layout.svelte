<script lang="ts">
  import type { Snippet } from "svelte";

  import "../app.css";

  import Header from "$lib/Header.svelte";
  import Footer from "$lib/Footer.svelte";

  interface LayoutProps {
    children: Snippet;
  }

  let { children }: LayoutProps = $props();
</script>

<svelte:head>
  <title>hncli</title>
  <meta name="description" content="hncli, a modern TUI for browsing Hacker News" />
</svelte:head>

<div class="w-full">
  <Header />
  <main class="flex-auto">
    {@render children()}
  </main>
  <Footer />
</div>

<style lang="postcss">
  @reference "tailwindcss";

  :global(html, body) {
    @apply w-full max-w-full overflow-x-hidden;
  }

  main {
    @apply w-full;
  }
</style>
