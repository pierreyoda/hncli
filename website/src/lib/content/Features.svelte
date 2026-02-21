<script lang="ts">
  import FeatureCard from "./FeatureCard.svelte";
  import ExternalLink from "$lib/ExternalLink.svelte";

  import BookLogo from "$lib/assets/book.svg";
  import RustLangLogo from "$lib/assets/rust.svg";
  import BookmarkLogo from "$lib/assets/bookmark.svg";
  import CommandLineLogo from "$lib/assets/command-line.svg";
</script>

<section id="features" class="website-container mt-6">
  <div class="features-container">
    <FeatureCard eyebrow="Built to last" title="Made in Rust">
      {#snippet image()}
        <img class="logo" alt="rustlang logo" src={RustLangLogo} />
      {/snippet}
      {#snippet description()}
        <span>
          Built in the fast and safe Rust programming language, with a unique architecture
          <ExternalLink href="https://www.newstackwhodis.com/blog/hncli-2-architecture">described here</ExternalLink>.
          <br /><br />hncli is first and foremost a labor of love, so questions or any kind of contribution are more
          than welcome on the
          <ExternalLink href="https://github.com/pierreyoda/hncli">GitHub repository</ExternalLink>.
        </span>
      {/snippet}
    </FeatureCard>
    <FeatureCard eyebrow="Read-Only" title="No sign-in required">
      {#snippet image()}
        <img class="logo" alt="book logo" src={BookLogo} />
      {/snippet}
      {#snippet description()}
        <span>
          No Hacker News account required: hncli does not need nor store your credentials.<br /><br />
        </span>
      {/snippet}
    </FeatureCard>
    <FeatureCard eyebrow="Well deserved pause?" title="Terminal only">
      {#snippet image()}
        <img class="logo" alt="terminal logo" src={CommandLineLogo} />
      {/snippet}
      {#snippet description()}
        <span>
          hncli works in most terminals, and one of the initial goals was to make technological watch on Hacker News
          easier at a glance. <br /> <br /> Check out the
          <ExternalLink href="https://www.newstackwhodis.com/blog/hncli-1-concept">vision article</ExternalLink> for further
          explanation.
        </span>
      {/snippet}
    </FeatureCard>
    <FeatureCard eyebrow="Navigation history" title="Pick up from where you left off">
      {#snippet image()}
        <img class="logo" alt="bookmark logo" src={BookmarkLogo} />
      {/snippet}
      {#snippet description()}
        <span> Go back to the latest read top-level comment on any topic previously visited. </span>
      {/snippet}
    </FeatureCard>
  </div>
</section>

<style lang="postcss">
  @reference "tailwindcss";

  .features-container {
    @apply grid grid-cols-1 gap-8 sm:grid-cols-2 md:grid-cols-2;
  }

  .logo {
    @apply absolute bottom-0 h-auto w-16;
  }
</style>
