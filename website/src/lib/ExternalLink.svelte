<script lang="ts">
  export let href: string;
  export let extraClass = "";
  export let underline = true;
</script>

<a class={extraClass} class:underline {href} target="_blank" rel="noreferrer noopener"><slot /></a>

<style lang="postcss">
  a {
    @apply text-hncli-dark-red transition-colors duration-200 hover:text-hncli-dark-red/60;
  }
</style>
