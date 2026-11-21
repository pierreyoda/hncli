<script lang="ts">
  import ChangelogEntry, { type ChangelogEntryProps } from "#lib/content/ChangelogEntry.svelte";
  import ChangelogImageResume from "#lib/assets/hncli-v0.5-resume.png";
  import HnCliDemoVideo from "#lib/assets/hncli-demo.mp4";

  const entries: readonly ChangelogEntryProps[] = [
    {
      tag: "0.5.0",
      media: "image",
      imageSrc: ChangelogImageResume,
      imageAlt: "Resume reading HN item feature",
      changes: [
        "Feature: Resume item reading history tab (see screenshot)",
        "Bugfix: fix comments count in home screen items metadata display",
        "Configuration: show items metadata in home screen by default",
        "Refactor: switch from crossterm to termina backend in ratatui",
        "Refactor: centralization of every rendered text",
      ],
    },
    {
      tag: "0.4.0",
      subtitle: "First beta tagged version of hncli.",
      media: "video",
      videoSrc: HnCliDemoVideo,
    },
  ];
</script>

<div class="changelog">
  {#each entries as { tag, ...props } (tag)}
    <ChangelogEntry {tag} {...props} />
  {/each}
</div>

<style lang="postcss">
  @reference "tailwindcss";

  .changelog {
    @apply flex flex-col gap-6;
    @apply mx-auto w-full bg-gray-900;
  }
</style>
