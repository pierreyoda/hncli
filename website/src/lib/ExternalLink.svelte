<script lang="ts">
  import type { Snippet } from "svelte";

  interface ExternalLinkProps {
    href: string;
    extraClass: string;
    /** True by default; */
    underline?: boolean;
    /** True by default. */
    red?: boolean;
    children: Snippet;
  }

  const { href, extraClass, underline = true, red = true, children }: ExternalLinkProps = $props();
</script>

<a class={extraClass} class:underline class:red {href} target="_blank" rel="noreferrer noopener">
  {@render children()}
</a>

<style lang="postcss">
  a {
    @apply transition-colors duration-200;
    &.red {
      @apply text-hncli-dark-red hover:text-hncli-dark-red/60;
    }
    &:not(.red) {
      @apply text-white hover:text-gray-200;
    }
  }
</style>
