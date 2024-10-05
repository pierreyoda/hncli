<script lang="ts">
  export let title: string;
  export let description: string;
  export let imageSrc: string;
  export let imageAlt: string;
  /** Reversed order of text and screenshot. Text on left by default. */
  export let reversed = false;
</script>

<div class="feature">
  <div class="flex flex-col lg:flex-row items-center" class:reversed>
    <div class="flex flex-col items-center lg:items-start">
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
