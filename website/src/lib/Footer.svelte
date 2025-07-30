<script lang="ts">
  const currentYear = new Date().getFullYear();
  // TODO: sticky bottom footer
</script>

<footer>
  <div class="text-center flex flex-col justify-center items-center w-full h-full">
    <p class="text">
      &copy; Copyright {currentYear} Pierre-Yves Diallo
    </p>
  </div>
</footer>

<style lang="postcss">
  @reference "../app.css";

  footer {
    @apply border-hncli-light-orange/50 border-t-4 bg-gray-900;
  }

  .text {
    @apply py-4 text-base font-medium tracking-tight text-white;
  }
</style>
