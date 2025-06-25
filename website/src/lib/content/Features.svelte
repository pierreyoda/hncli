<script lang="ts">
  import FeatureCard from "./FeatureCard.svelte";
  import ExternalLink from "$lib/ExternalLink.svelte";

  import BookLogo from "$lib/assets/book.svg";
  import RustLangLogo from "$lib/assets/rust.svg";
  import CommandLineLogo from "$lib/assets/command-line.svg";

  // TODO: use grid instead of flex if needed
</script>

<section id="features" class="website-container">
  <div class="grid-container">
    <FeatureCard small eyebrow="Built to last" title="Made in Rust with a custom architecture" extraClass="col-span-3">
      {#snippet image()}
        <img slot="image" class="screenshot" alt="hncli source code excerpt" src={RustLangLogo} />
      {/snippet}
      {#snippet description()}
        <span slot="description">
          Built in the fast and safe Rust programming language, with a unique architecture
          <ExternalLink
            href="https://www.newstackwhodis.com/blog/hncli-2-architecture"
            extraClass="font-semibold text-hncli-dark-red">described here</ExternalLink
          >. <br /><br />hncli is first and foremost a labor of love, so questions or any kind of contribution are more
          than welcome on the
          <ExternalLink href="https://github.com/pierreyoda/hncli" extraClass="font-semibold text-hncli-dark-red"
            >GitHub repository</ExternalLink
          >.
        </span>
      {/snippet}
    </FeatureCard>
    <FeatureCard
      small
      eyebrow="Read-Only"
      title="No sign-in required"
      textDescription="No Hacker News account required: hncli does not need nor store your credentials in any way whatsoever."
      extraClass="col-span-3"
    >
      {#snippet image()}
        <img slot="image" class="screenshot w-24 h-auto" alt="book logo" src={BookLogo} />
      {/snippet}
    </FeatureCard>
    <FeatureCard small eyebrow="Well deserved pause?" title="Terminal only" extraClass="">
      {#snippet image()}
        <img slot="image" class="screenshot w-24 h-auto" alt="terminal only" src={CommandLineLogo} />
      {/snippet}
      {#snippet description()}
        <span slot="description">
          hncli works in most terminals, and one of the initial goals was to make technological watch easier at a
          glance. <br /> <br /> As described in the
          <ExternalLink
            href="https://www.newstackwhodis.com/blog/hncli-1-concept#project-description"
            extraClass="font-semibold text-hncli-dark-red">first article</ExternalLink
          >, refactoring for answering to topics or comments from hncli would be really difficult - even for me - both
          in terms of engineering and UX.
        </span>
      {/snippet}
    </FeatureCard>
  </div>
</section>

<style lang="postcss">
  .grid-container {
    @apply mt-10 grid grid-cols-1 gap-4 sm:mt-16 md:grid-cols-2;
  }

  .screenshot {
    @apply absolute bottom-0;
  }
</style>
