<script lang="ts">
  export let href: string;
  export let extraClass = "";
</script>

<a class={extraClass} {href} target="_blank" rel="noreferrer noopener"><slot /></a>

<style lang="postcss">
  a {
    @apply text-hncli-dark-red underline transition-colors duration-200 hover:text-hncli-dark-red/60;
  }
</style>
