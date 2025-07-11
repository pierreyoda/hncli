<script lang="ts">
  import FeatureFocus from "./FeatureFocus.svelte";

  import CommentsScreenshot from "$lib/assets/hncli-comments.webp";
  import SettingsScreenshot from "$lib/assets/hncli-settings.webp";
  import ObsoleteSearchScreenshot from "$lib/assets/hncli-limited-search.png";
  import ContextualHelpScreenshot from "$lib/assets/hncli-contextual-help.webp";
</script>

<section id="screenshots" aria-label="hncli screenshots" class="features-container">
  <div class="website-container">
    <FeatureFocus
      title="Browse every comment or sub-comment, on any story"
      description="On the Home, Ask HN and Show HN (usernames erased here)."
      imageSrc={CommentsScreenshot}
      imageAlt="hncli sub-comment display"
    />
    <FeatureFocus
      title="Contextual help on all screens"
      description="If enabled, contextual help will display the most common commands on any given screen."
      imageSrc={ContextualHelpScreenshot}
      imageAlt="hncli contextual help on Show Hacker News list screen"
      reversed
    />
    <FeatureFocus
      title="Customize your experience"
      description="Persistent settings, stored in the OS-appropriate folder."
      imageSrc={SettingsScreenshot}
      imageAlt="hncli settings screen"
    />
    <FeatureFocus
      title="Search throughout hncli (posts, comments, users)"
      description="After heavy efforts on my part, hn.algolia unfortunately shut down almost all its returned data. Here is an example of what still works."
      imageSrc={ObsoleteSearchScreenshot}
      imageAlt="hncli search screen (limited by hn.algolia.com"
      reversed
      obsolete
    />
  </div>
</section>

<style lang="postcss">
  .features-container {
    @apply mx-auto w-full bg-gray-900;
  }
</style>
