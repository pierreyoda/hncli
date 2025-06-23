<script lang="ts">
  const currentYear = new Date().getFullYear();
</script>

<footer>
  <div class="text-center flex flex-col justify-center items-center w-full h-full">
    <p class="text">
      &copy; Copyright {currentYear} Pierre-Yves Diallo
    </p>
  </div>
</footer>

<style lang="postcss">
  footer {
    @apply border-t-4 border-hncli-light-orange/50 bg-gray-500;
  }

  .text-container {
    @apply flex h-full w-full flex-col items-center justify-center text-center;
  }
  .text {
    @apply py-4 text-base font-medium tracking-tight text-white;
  }
</style>
