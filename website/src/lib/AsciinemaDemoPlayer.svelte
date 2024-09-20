<script lang="ts">
  import { onMount } from "svelte";

  let container: HTMLDivElement;
  let extraClass = "";

  onMount(async () => {
    // dynamic import since this module required "document" to be available
    const AsciinemaPlayer = await import("asciinema-player");
    AsciinemaPlayer.create("https://asciinema.org/a/znCZULtFhQjBan32MVHWzuzda.cast", container, {
      loop: true,
      poster: "npt:3",
      startAt: 3,
      theme: "dracula",
    });
  });

  $: containerClass = `video-container ${extraClass}`;
</script>

<div bind:this={container} class={containerClass} />

<style lang="postcss">
  .video-container {
    @apply mx-auto h-auto max-w-[1200px];
  }
</style>
