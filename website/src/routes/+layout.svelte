<script lang="ts">
  import "../app.css";

  import Header from "$lib/Header.svelte";
  import Footer from "$lib/Footer.svelte";
</script>

<div>
  <Header />
  <main class="flex-auto">
    <slot />
  </main>
  <Footer />
</div>
