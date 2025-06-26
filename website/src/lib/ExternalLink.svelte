<script lang="ts">
  import type { Snippet } from "svelte";

  interface ExternalLinkProps {
    href: string;
    extraClass: string;
    /** True by default; */
    underline?: boolean;
    /** True by default. */
    redHighlight?: boolean;
    children: Snippet;
  }

  const { href, extraClass, underline = true, redHighlight = true, children }: ExternalLinkProps = $props();
</script>

<a class={extraClass} class:underline class:red={redHighlight} {href} target="_blank" rel="noreferrer noopener">
  {@render children()}
</a>

<style lang="postcss">
  a {
    @apply text-hncli-dark-red transition-colors duration-200;
    &.red {
      @apply text-hncli-dark-red hover:text-hncli-dark-red/60;
    }
  }
</style>
