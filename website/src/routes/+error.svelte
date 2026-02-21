<script lang="ts">
  import { page } from "$app/state";
</script>

<section class="website-container error-container">
  <p class="error-title">404</p>
  <h1 class="error-message">{page.error?.message ?? "Page not found"}</h1>
  <p class="error-description">Sorry, we couldn&apos;t find the page you&apos;re looking for.</p>
</section>

<style lang="postcss">
  @reference "tailwindcss";

  .error-container {
    @apply flex min-h-full flex-col items-center justify-center text-center;
    @apply mx-auto bg-white pt-32 pb-8;
  }

  .error-title {
    @apply mt-4 text-5xl font-semibold tracking-tight text-balance text-gray-900 sm:text-7xl;
  }
  .error-message {
    @apply mt-auto grow p-8;
    @apply text-4xl font-semibold text-gray-600;
  }
  .error-description {
    @apply mt-6 text-lg font-medium text-pretty text-gray-600 sm:text-xl/8;
  }
</style>
