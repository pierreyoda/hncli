<script lang="ts">
  import Button from "$lib/Button.svelte";
  import HeroBackgroundIllustration from "./HeroBackgroundIllustration.svelte";

  import screenshot from "$lib/assets/home-screen.png";
</script>

<div class="overflow-hidden py-20 sm:py-32 lg:pb-32 xl:pb-36">
  <div class="website-container">
    <div class="lg:grid lg:grid-cols-12 lg:gap-x-8 lg:gap-y-20">
      <div class="relative z-10 mx-auto mx-w-2xl lg:col-span-7 lg:max-w-none lg:pt-6 xl:col-span-6">
        <h2 class="text-4xl font-medium tracking-tight text-gray-900">Read Hacker News in your terminal</h2>
        <p class="mt-6 text-lg text-gray-600">
          A convenient - and discreet - way to browse Y Combinator's Hacker News right inside your terminal.
        </p>
        <div class="mt-8 flex">
          <Button variant="solid" color="red" href="https://news.ycombinator.com" externalLink>Hacker News</Button>
        </div>
      </div>
      <div class="relative mt-10 sm:mt-20 lg:col-span-5 lg:row-span-2 lg:mt-0 xl:col-span-6">
        <HeroBackgroundIllustration
          id="hero-background-illustration"
          extraClass="absolute left-1/2 top-4 h-[1026px] w-[1026px] -translate-x-1/3 stroke-gray-300/70 [mask-image:linear-gradient(to_bottom,white_20%,transparent_75%)] sm:top-16 sm:-translate-x-1/2 lg:-top-16 lg:ml-12 xl:-top-14 xl:ml-0"
        />
        <div
          class="-mx-4 h-[448px] px-9 [mask-image:linear-gradient(to_bottom,white_60%,transparent)] sm:mx-0 lg:absolute lg:-inset-x-10 lg:-bottom-20 lg:-top-10 lg:h-auto lg:px-0 lg:pt-10 xl:-bottom-32"
        >
          <img class="mx-auto max-w-[42rem] h-auto" src={screenshot} alt="hncli home screen" />
        </div>
      </div>
    </div>
  </div>
</div>
