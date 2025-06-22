<script lang="ts">
  import ClipboardCommandLines from "./ClipboardCommandLines.svelte";

  type InstallationOption = {
    title: string;
    lines: readonly string[];
  };

  const options: readonly InstallationOption[] = [
    { title: "With Docker", lines: ["docker build -t hncli .  && docker run -it hncli"] },
    { title: "Compile with Rust (rustup recommended)", lines: ["cargo run --release"] },
    // { title: "Build from Rust's crates.io", lines: ["cargo install hncli"] },
    // { title: "Install with Homebrew", lines: ["brew install hncli"] },
  ];
</script>

<section id="install" aria-label="Install hncli" class="pt-12 pb-3">
  <div class="website-container">
    <div class="text-center">
      <h2 class="title">How to use</h2>
      <p class="subtitle">(more ways to come)</p>
    </div>
    <div class="options-container">
      {#each options as { title, lines }}
        <div class="option-container">
          <h3 class="option-title">{title}:</h3>
          <ClipboardCommandLines {lines} />
        </div>
      {/each}
    </div>
  </div>
</section>

<style lang="postcss">
  .title {
    @apply mb-1 text-4xl font-semibold text-hncli-dark-red;
  }
  .subtitle {
    @apply text-sm font-medium text-hncli-dark-red/60;
  }

  .options-container {
    @apply flex w-full flex-col py-6 md:flex-row;
  }

  .option-container {
    @apply flex flex-col items-center justify-around pt-2;
    .option-title {
      @apply pb-1 text-center text-lg font-medium text-gray-500;
    }
  }
</style>
