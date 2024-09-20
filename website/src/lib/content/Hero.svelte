<script lang="ts">
  import Button from "$lib/Button.svelte";
  import HeroBackgroundIllustration from "./HeroBackgroundIllustration.svelte";

  import AsciinemaDemoPlayer from "$lib/AsciinemaDemoPlayer.svelte";
</script>

<div class="overflow-hidden pb-4">
  <div class="website-container flex flex-col mx-auto mx-w-2xl">
    <div class="text-center mx-auto pb-6">
      <h2 class="text-4xl font-semibold tracking-tight text-gray-900">Read Hacker News in your terminal</h2>
      <p class="mt-6 text-lg text-gray-600 font-medium">
        A convenient - and discreet - way to browse Y Combinator's Hacker News right inside your terminal.
      </p>
      <div class="mt-8 flex">
        <Button variant="solid" color="red" href="https://news.ycombinator.com" externalLink>Hacker News</Button>
      </div>
    </div>
    <div class="w-full relative">
      <HeroBackgroundIllustration
        id="hero-background-illustration"
        extraClass="absolute left-1/2 top-4 h-[1026px] w-[1026px] -translate-x-1/3 stroke-gray-300/70 [mask-image:linear-gradient(to_bottom,white_20%,transparent_75%)] sm:top-16 sm:-translate-x-1/2 lg:-top-16 lg:ml-12 xl:-top-14 xl:ml-0"
      />

      <AsciinemaDemoPlayer extraClass="mx-auto" />
      <div
        class="-mx-4 h-[448px] px-9 [mask-image:linear-gradient(to_bottom,white_60%,transparent)] sm:mx-0 lg:absolute lg:-inset-x-10 lg:-bottom-20 lg:-top-10 lg:h-auto lg:px-0 lg:pt-10 xl:-bottom-32"
      ></div>
    </div>
  </div>
</div>
