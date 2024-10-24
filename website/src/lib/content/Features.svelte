<script lang="ts">
  import FeatureCard from "./FeatureCard.svelte";
  import ExternalLink from "$lib/ExternalLink.svelte";

  import BookLogo from "$lib/assets/book.svg";
  import RustLangLogo from "$lib/assets/rust.svg";
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
          >. hncli is first and foremost a labor of love, so questions or any kind of contribution are more than welcome
          on the
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
  </div>
</section>

<style lang="postcss">
  .grid-container {
    @apply mt-10 grid grid-cols-1 gap-4 sm:mt-16 lg:grid-cols-6;
  }

  .screenshot {
    @apply absolute bottom-0;
  }
</style>
