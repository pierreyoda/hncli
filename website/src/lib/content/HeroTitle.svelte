<script lang="ts">
  import ExternalLink from "$lib/ExternalLink.svelte";
</script>

<section id="hero-title" aria-label="hero section title" class="section-container">
  <h2 class="text-4xl font-semibold tracking-tight text-gray-800">
    Browse
    <ExternalLink href="https://news.ycombinator.com/" extraClass="no-underline">Hacker News</ExternalLink>&nbsp;from
    your terminal.
  </h2>
  <p class="mt-6 text-lg text-gray-600 font-medium">
    Convenient, fast, and will run well on most computers, even old ones.
  </p>
</section>

<style lang="postcss">
  .section-container {
    @apply py-6 text-center;
    @apply bg-hncli-light-orange/20;
  }
</style>
