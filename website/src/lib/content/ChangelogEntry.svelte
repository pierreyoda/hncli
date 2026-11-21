<script module lang="ts">
  export type ChangelogEntryProps = {
    /** Format: "0.5.0" */
    tag: string;
    subtitle?: string;
    changes?: readonly string[];
  } & (
    | {
        media: "image";
        imageSrc: string;
        imageAlt: string;
      }
    | {
        media: "video";
        videoSrc: string;
      }
  );
</script>

<script lang="ts">
  import { resolve } from "$app/paths";

  const { tag, subtitle, changes, ...props }: ChangelogEntryProps = $props();
</script>

<article class="entry">
  <div class="title">
    <a href={resolve(`changelog#${tag}`)} class="link">#</a>
    <h2 class="tag" id={tag}>v{tag}</h2>
  </div>
  {#if subtitle}
    <h3 class="subtitle">{subtitle}</h3>
  {/if}
  {#if props.media === "image"}
    <img class="media" alt={props.imageAlt} src={props.imageSrc} />
  {:else}
    <video class="media" src={props.videoSrc} autoplay muted playsinline loop></video>
  {/if}
  {#if (changes?.length ?? 0) > 0}
    <ul class="list-disc text-left">
      {#each changes as change (change)}
        <li>{change}</li>
      {/each}
    </ul>
  {/if}
</article>

<style lang="postcss">
  @reference "tailwindcss";

  .entry {
    @apply flex flex-col items-center gap-2 py-6 text-center text-white;

    .title {
      @apply flex items-center gap-2;
      @apply font-mono text-2xl font-semibold tracking-tight;
      .link {
        @apply text-gray-300 transition-colors duration-500 hover:text-gray-500;
      }
    }
    .media {
      @apply h-auto w-180;
    }
  }
</style>
