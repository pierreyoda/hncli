<script lang="ts">
  import ExternalLink from "./ExternalLink.svelte";

  import GitHubLogo from "$lib/assets/github.svg";
  import HnCliTextLogo from "$lib/assets/hncli-logo-text.png";
</script>

<header class="website-container">
  <nav>
    <a href="/" aria-label="Home" class="flex flex-col items-center">
      <img alt="Wrench icon" src={HnCliTextLogo} class="max-w-40 h-auto" />
      <h1 class="hidden">hncli</h1>
    </a>
  </nav>
  <div class="flex items-center">
    <ExternalLink
      href="https://www.newstackwhodis.com/blog"
      underline={false}
      extraClass="mr-6 no-underline text-xl font-medium text-hncli-dark-red">Blog</ExternalLink
    >
    <ExternalLink href="https://github.com/pierreyoda/hncli" extraClass="cursor-pointer">
      <img alt="GitHub logo" src={GitHubLogo} class="w-10 h-10" />
    </ExternalLink>
  </div>
</header>

<style lang="postcss">
  header {
    @apply flex items-center justify-between py-8 pl-10 md:pl-0;
  }
</style>
