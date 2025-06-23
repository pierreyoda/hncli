<script lang="ts">
  import Button from "$lib/Button.svelte";
</script>

<section id="contribute" aria-label="Contribute to hncli" class="bg-gray-900 py-20 sm:py-3 md:py-12 mt-12">
  <div class="website-container">
    <div class="flex flex-col md:flex-row items-center justify-between">
      <div class="half-section">
        <h2 class="text-white text-4xl font-medium tracking-tight">Any issue? Wanting to contribute?</h2>
      </div>
      <div class="half-section mt-12 md:mt-0 text-4xl">
        <Button
          type="link"
          variant="solid"
          color="red"
          href="https://github.com/pierreyoda/hncli/issues/new"
          externalLink
          extraClass="!text-xl"
        >
          Open a new issue in GitHub</Button
        >
      </div>
    </div>
  </div>
</section>

<style lang="postcss">
  .half-section {
    @apply flex w-1/2 flex-col items-center justify-start text-center md:text-left;
  }
</style>
