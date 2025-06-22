<script lang="ts">
  import ClipboardCommandLines from "./ClipboardCommandLines.svelte";

  type InstallationOption = {
    title: string;
    lines: readonly string[];
  };

  const options: readonly InstallationOption[] = [
    { title: "With Docker", lines: ["docker build -t hncli .  && docker run -it hncli"] },
    { title: "Compile with Rust (rustup recommended", lines: ["cargo run --release"] },
    // { title: "Build from Rust's crates.io", lines: ["cargo install hncli"] },
    // { title: "Install with Homebrew", lines: ["brew install hncli"] },
  ];
</script>

<section id="install" aria-label="Install hncli" class="pt-12 pb-3">
  <div class="website-container">
    <h2 class="title">How to run (more ways to come)</h2>
    <div class="options-container">
      {#each options as { title, lines }}
        <div class="option-container">
          <h3 class="option-title">{title}:</h3>
          <ClipboardCommandLines {lines} extraClass="w-5/6" />
        </div>
      {/each}
    </div>
  </div>
</section>

<style lang="postcss">
  .title {
    @apply mb-8 text-center text-4xl font-semibold text-hncli-dark-red;
  }

  .options-container {
    @apply grid w-full grid-cols-1 grid-rows-2 gap-6 lg:grid-cols-2;
  }

  .option-container {
    @apply flex flex-col items-start;
    .option-title {
      @apply pb-6 text-lg font-medium text-gray-900 lg:text-xl;
    }
  }
</style>
