<script lang="ts">
  import ExternalLink from "$lib/ExternalLink.svelte";
</script>

<section id="hero-title" aria-label="hero section title">
  <h2 class="text-4xl font-semibold tracking-tight text-gray-800">
    Browse
    <ExternalLink extraClass="underline" href="https://news.ycombinator.com/">Hacker News</ExternalLink>&nbsp;from your
    terminal.
  </h2>
  <p class="mt-6 text-lg text-gray-600 font-medium">
    Convenient, fast, and will run well on most computers, even old ones.
  </p>
  <p class="text-xl font-medium tracking-tight text-gray-700 mt-4">
    <ExternalLink
      extraClass="no-underline mt-4"
      href="https://www.newstackwhodis.com/blog/hncli-1-concept#an-hacker-news-terminal-ui-concept-and-road-to-release"
    >
      Concept, vision and UX</ExternalLink
    >
  </p>
</section>

<style lang="postcss">
  @reference "../../app.css";

  #hero-title {
    @apply bg-hncli-light-orange/20 py-6 text-center;
  }
</style>
