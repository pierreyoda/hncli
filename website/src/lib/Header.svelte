<script lang="ts">
  import { resolve } from "$app/paths";

  import ExternalLink from "./ExternalLink.svelte";

  import GitHubLogo from "$lib/assets/github.svg";
  import HnCliTextLogo from "$lib/assets/hncli-logo-text.png";
</script>

<header>
  <nav>
    <a href={resolve("/")} aria-label="Home" class="flex flex-col items-center">
      <img alt="hncli logo" src={HnCliTextLogo} class="max-w-40 h-auto" />
      <h1 class="hidden">hncli</h1>
    </a>
  </nav>
  <div class="flex items-center sm:pr-12 sm:justify-around pt-4 sm:pt-0">
    <ExternalLink
      href="https://www.newstackwhodis.com/blog/?tag=Rust"
      underline={false}
      red={false}
      extraClass="mr-6 no-underline text-xl font-medium">Blog</ExternalLink
    >
    <ExternalLink href="https://github.com/pierreyoda/hncli" red={false} extraClass="cursor-pointer">
      <img alt="GitHub logo" src={GitHubLogo} class="w-10 h-10 ml-3 md:ml-6" />
    </ExternalLink>
  </div>
</header>

<style lang="postcss">
  @reference "../app.css";

  header {
    @apply bg-red-400;
    @apply flex flex-col items-center justify-between px-12 py-8 pl-10 sm:flex-row md:pl-12;
  }
</style>
