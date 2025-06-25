<script lang="ts">
  // TODO: better MP4 compression if possible
  import HnCliDemoVideo from "$lib/assets/hncli-demo.mp4";
</script>

<section id="hero" aria-label="Hero section">
  <div class="website-container mx-w-2xl">
    <div class="text-center mx-auto pb-6">
      <div class="mt-8 w-full relative">
        <video src={HnCliDemoVideo} autoplay muted playsinline loop class="demo-video"></video>
      </div>
    </div>
  </div>
</section>

<style lang="postcss">
  #hero {
    @apply overflow-hidden bg-gray-800;
  }

  .website-container {
    @apply mx-auto flex flex-col;
  }

  .demo-video {
    @apply z-10 mx-auto h-auto w-[720px];
  }
</style>
