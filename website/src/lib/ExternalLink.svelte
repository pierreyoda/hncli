<script lang="ts">
  export let href: string;
  export let extraClass = "";

  $: linkClass = `underline ${extraClass}`;
</script>

<a class={linkClass} {href} target="_blank" rel="noreferrer noopener"><slot /></a>
