<script>
  import { page } from "$app/stores";
</script>

<div class="website-container flex items-center justify-center">
  <h1 class="text-4xl p-16 text-gray-600">{$page.error?.message}</h1>
</div>
