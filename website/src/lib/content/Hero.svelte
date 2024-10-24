<script lang="ts">
  import Button from "$lib/Button.svelte";

  import HnCliDemoVideo from "$lib/assets/hncli-demo.mp4";
</script>

<section id="hero" aria-label="Hero section" class="overflow-hidden pb-4">
  <div class="website-container flex flex-col mx-auto mx-w-2xl">
    <div class="text-center mx-auto pb-6">
      <h2 class="text-4xl font-semibold tracking-tight text-gray-900">Read Hacker News in your terminal</h2>
      <p class="mt-6 text-lg text-gray-600 font-medium">
        A convenient - and discreet - way to browse Y Combinator's Hacker News right inside your terminal.
      </p>
      <div class="mt-8 mx-auto">
        <Button variant="solid" color="red" href="https://news.ycombinator.com" externalLink>Hacker News</Button>
      </div>
      <div class="mt-8 w-full relative">
        <video src={HnCliDemoVideo} autoplay muted playsinline loop class="demo-video"></video>
      </div>
    </div>
  </div>
</section>

<style lang="postcss">
  .demo-video {
    @apply z-10 mx-auto h-auto w-[720px];
  }
</style>
