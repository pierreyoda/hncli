<script lang="ts">
  import FeatureCard from "./FeatureCard.svelte";
  import ExternalLink from "$lib/ExternalLink.svelte";

  import SettingsScreenshot from "$lib/assets/comments.png";
  import RustLangLogo from "$lib/assets/rust.svg";
  import SourceCodeScreenshot from "$lib/assets/github.svg";
  import ContextualHelpScreenshot from "$lib/assets/contextual-help.png";
</script>

<div class="website-container">
  <div class="grid-container">
    <FeatureCard
      fadeTop
      eyebrow="UX"
      title="Contextual Help"
      description="If enabled, a dynamic contextual help provides the current screen's shortcuts in a clear fashion."
      extraClass="rounded-tr-4xl lg:col-span-3 lg:rounded-tl-4xl"
    >
      <img slot="image" class="screenshot" alt="hncli contextual help" src={ContextualHelpScreenshot} />
    </FeatureCard>
    <FeatureCard
      fadeTop
      eyebrow="Customization"
      title="Dynamic settings"
      description="Persistent settings, stored in the OS-appropriate folder."
      extraClass="lg:col-span-3 lg:rounded-tr-4xl"
    >
      <img slot="image" class="screenshot" alt="hncli settings screen" src={SourceCodeScreenshot} />
    </FeatureCard>
    <FeatureCard
      small
      eyebrow="Built to last"
      title="Made in Rust with a custom architecture"
      extraClass="lg:col-span-4 lg:rounded-tr-4xl"
    >
      <img slot="image" class="screenshot" alt="hncli source code excerpt" src={RustLangLogo} />
      <span slot="description">
        Built in the fast and safe Rust programming language, with a unique architecture
        <ExternalLink
          href="https://www.newstackwhodis.com/blog/hncli-2-architecture"
          extraClass="font-semibold text-hncli-dark-red">described here</ExternalLink
        >. hncli is first and foremost a labor of love and questions or any kind of contribution arew more than welcome
        on the
        <ExternalLink href="https://github.com/pierreyoda/hncli" extraClass="font-semibold text-hncli-dark-red"
          >GitHub repository</ExternalLink
        >.
      </span>
    </FeatureCard>
    <FeatureCard
      small
      eyebrow="Read-Only"
      title="No sign-in required"
      description="No Hacker News account required: hncli does not need nor store your credentials in any way whatsoever."
      extraClass="lg:col-span-2 lg:rounded-tr-4xl"
    ></FeatureCard>
  </div>
</div>

<style lang="postcss">
  .grid-container {
    @apply mt-10 grid grid-cols-1 gap-4 sm:mt-16 lg:grid-cols-6;
  }

  .screenshot {
    @apply absolute bottom-0;
  }
  .link {
    @apply font-semibold text-hncli-dark-red underline;
  }
</style>
