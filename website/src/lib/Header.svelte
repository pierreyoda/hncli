<script lang="ts">
  import ExternalLink from "./ExternalLink.svelte";

  import GitHubLogo from "$lib/assets/github.svg";
  import HnCliTextLogo from "$lib/assets/hncli-logo-text.png";
</script>

<header>
  <nav class="pl-12">
    <a href="/" aria-label="Home" class="flex flex-col items-center">
      <img alt="Wrench icon" src={HnCliTextLogo} class="max-w-40 h-auto" />
      <h1 class="hidden">hncli</h1>
    </a>
  </nav>
  <div class="flex items-center pr-12">
    <ExternalLink
      href="https://www.newstackwhodis.com/blog"
      underline={false}
      redHighlight={false}
      extraClass="mr-6 no-underline text-xl font-medium text-white">Blog</ExternalLink
    >
    <ExternalLink href="https://github.com/pierreyoda/hncli" extraClass="cursor-pointer">
      <img alt="GitHub logo" src={GitHubLogo} class="w-10 h-10" />
    </ExternalLink>
  </div>
</header>

<style lang="postcss">
  header {
    @apply bg-red-400;
    @apply flex items-center justify-between py-8 pl-10 md:pl-0;
  }
</style>
