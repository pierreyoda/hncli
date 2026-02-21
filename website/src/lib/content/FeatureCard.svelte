<script lang="ts">
  import type { Snippet } from "svelte";

  interface FeatureCardProps {
    eyebrow: string;
    title: string;
    textDescription?: string;
    extraClass?: string;
    fadeTop?: boolean;
    fadeBottom?: boolean;
    large?: boolean;
    image: Snippet;
    description?: Snippet;
  }

  const {
    eyebrow,
    title,
    textDescription,
    extraClass,
    fadeTop = false,
    fadeBottom = false,
    large = false,
    image,
    description,
  }: FeatureCardProps = $props();

  const containerClass = $derived<string>(
    [
      extraClass ?? "",
      "relative flex flex-col overflow-hidden rounded-lg",
      "bg-white shadow-xs ring-1 ring-black/5",
    ].join(" "),
  );
</script>

<div class={containerClass}>
  <div class="image-container" class:large>
    {@render image()}
    {#if fadeTop}
      <div class="absolute inset-0 bg-linear-to-b from-white to-50%"></div>
    {/if}
    {#if fadeBottom}
      <div class="absolute inset-0 bg-linear-to-t from-white to-50%"></div>
    {/if}
  </div>
  <div class="relative p-10 pt-6">
    <h3 class="eyebrow">
      {eyebrow}
    </h3>
    <p class="title">
      {title}
    </p>
    <p class="description">
      {#if textDescription}
        {textDescription}
      {:else}
        {@render description?.()}
      {/if}
    </p>
  </div>
</div>

<style lang="postcss">
  @reference "tailwindcss";

  .image-container {
    @apply relative flex h-20 shrink-0 items-center pl-10;
    &.large {
      @apply h-40;
    }
  }
  .eyebrow {
    @apply font-mono text-base font-semibold tracking-widest text-gray-500 uppercase;
  }
  .title {
    @apply mt-1 text-3xl font-medium tracking-tight text-gray-950;
  }
  .description {
    @apply mt-4 max-w-150 text-justify text-base/6 text-gray-600;
  }
</style>
