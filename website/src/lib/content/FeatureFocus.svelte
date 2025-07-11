<script lang="ts">
  interface FeatureFocusProps {
    title: string;
    description: string;
    imageSrc: string;
    imageAlt: string;
    /** Reversed order of text and screenshot. Text on left by default. */
    reversed?: boolean;
    /** False by default. */
    obsolete?: boolean;
  }

  const { title, description, imageSrc, imageAlt, reversed = false, obsolete = false }: FeatureFocusProps = $props();
</script>

<div class="feature">
  <div class="flex flex-col lg:flex-row items-center" class:reversed class:obsolete>
    <div class="flex flex-col items-center lg:items-start">
      {#if obsolete}
        <span class="text-gray-300 mb-2 text-sm font-bold tracking-tighter">Obsolete</span>
      {/if}
      <h2 class="title">{title}</h2>
      <p class="description">{description}</p>
    </div>
    <img class="image" alt={imageAlt} src={imageSrc} />
  </div>
</div>

<style lang="postcss">
  .feature {
    @apply bg-gray-900 py-6 text-center md:py-20 lg:text-left;

    .reversed {
      @apply lg:flex-row-reverse;
    }
    .obsolete {
      @apply opacity-70;
    }

    .title {
      @apply text-3xl font-medium tracking-tight text-white;
    }
    .description {
      @apply mt-2 text-lg text-gray-400;
    }
    .image {
      @apply h-auto w-[720px];
    }
  }
</style>
