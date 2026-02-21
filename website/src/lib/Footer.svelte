<script lang="ts">
  const currentYear = new Date().getFullYear();
</script>

<footer>
  <div class="text-center flex flex-col justify-center items-center w-full">
    <p class="text">
      &copy; Copyright {currentYear} Pierre-Yves Diallo
    </p>
  </div>
</footer>

<style lang="postcss">
  @reference "../app.css";

  footer {
    @apply flex flex-col items-center bg-gray-900;
  }

  .text {
    @apply py-4 font-medium text-white;
  }
</style>
