<script lang="ts">
  import Button from "$lib/Button.svelte";
  import ExternalLink from "$lib/ExternalLink.svelte";
</script>

<section id="hero-title" aria-label="hero section title" class="section-container">
  <h2 class="text-4xl font-semibold tracking-tight text-gray-800">
    Browse from your own terminal.
    <ExternalLink href="https://news.ycombinator.com/" extraClass="font-semibold">HackerNews</ExternalLink>from your
    terminal.
  </h2>
  <p class="mt-6 text-lg text-gray-600 font-medium">
    $A convenient - and discreet - way of browsing Y Combinator's Hacker News right inside your terminal.
  </p>
  <div class="mt-8 mx-auto">
    <Button variant="solid" color="red" href="https://news.ycombinator.com" externalLink>Hacker News</Button>
  </div>
</section>

<style lang="postcss">
  .section-container {
    @apply bg-hncli-light-orange/50 text-center;
  }
</style>
