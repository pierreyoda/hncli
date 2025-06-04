<script lang="ts">
  import type { Snippet } from "svelte";

  interface FeatureCardProps {
    eyebrow: string;
    title: string;
    textDescription?: string;
    extraClass?: string;
    fadeTop?: boolean;
    fadeBottom?: boolean;
    small?: boolean;
    image: Snippet;
    description?: Snippet;
  }

  const {
    eyebrow,
    title,
    textDescription,
    extraClass = "",
    fadeTop = false,
    fadeBottom = false,
    small = false,
    image: imageSlot,
    description,
  }: FeatureCardProps = $props();

  const containerClass = $derived<string>(
    [
      extraClass ?? "",
      small ? "grid-rows-1" : "lg:grid-rows-2",
      "group relative flex flex-col overflow-hidden rounded-lg",
      "bg-white shadow-xs ring-1 ring-black/5",
    ].join(" "),
  );
</script>

<div class={containerClass}>
  <div class="image-container" class:large={!small}>
    {@render imageSlot()}
    {#if fadeTop}
      <div class="absolute inset-0 bg-linear-to-b from-white to-50%"></div>
    {/if}
    {#if fadeBottom}
      <div class="absolute inset-0 bg-linear-to-t from-white to-50%"></div>
    {/if}
  </div>
  <div class="relative p-10">
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
  .image-container {
    @apply relative flex h-40 shrink-0 items-center pl-10;
    &.large {
      @apply h-80;
    }
  }
  .eyebrow {
    @apply font-mono text-base font-semibold uppercase tracking-widest text-gray-500;
  }
  .title {
    @apply mt-1 text-3xl font-medium tracking-tight text-gray-950;
  }
  .description {
    @apply mt-2 max-w-[600px] text-base/6 text-gray-600;
  }
</style>
