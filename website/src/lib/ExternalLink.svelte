<script lang="ts">
  import type { Snippet } from "svelte";

  interface ExternalLinkProps {
    href: string;
    extraClass: string;
    /** True by default; */
    underline?: boolean;
    children: Snippet;
  }

  const { href, extraClass, underline = true, children }: ExternalLinkProps = $props();
</script>

<a class={extraClass} class:underline {href} target="_blank" rel="noreferrer noopener">
  {@render children()}
</a>

<style lang="postcss">
  a {
    @apply text-hncli-dark-red transition-colors duration-200 hover:text-hncli-dark-red/60;
  }
</style>
