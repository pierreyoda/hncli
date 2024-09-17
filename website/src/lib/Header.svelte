<script lang="ts">
  import GitHubLogo from "$lib/assets/github.svg";
  import HnCliTextLogo from "$lib/assets/hncli-logo-text.png";

  type NavbarLink = {
    label: string;
    href: string;
  };
  const navbarLinks: readonly NavbarLink[] = [{ label: "Features", href: "/#features" }];
</script>

<header class="website-container flex items-center justify-between">
  <div />
  <nav>
    <a href="/" aria-label="Home" class="flex items-center">
      <img alt="Wrench icon" src={HnCliTextLogo} class="max-w-40 h-auto" />
      <h1 class="hidden">hncli</h1>
    </a>
  </nav>
  <a href="https://github.com/pierreyoda/hncli" title="hncli GitHub repository">
    <img alt="GitHub logo" src={GitHubLogo} class="w-10 h-10" />
  </a>
</header>

<style lang="postcss">
  header {
    @apply py-8;
  }

  nav {
    a {
      @apply no-underline;
    }
  }
</style>
