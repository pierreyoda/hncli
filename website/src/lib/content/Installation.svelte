<script lang="ts">
  import ClipboardCommandLines from "./ClipboardCommandLines.svelte";

  type InstallationOption = {
    title: string;
    lines: readonly string[];
  };

  // tTODO: switch to grid when more options
  const options: readonly InstallationOption[] = [
    { title: "With Docker", lines: ["docker build -t hncli .  && docker run -it hncli"] },
    { title: "With Rust toolchain", lines: ["cargo run --release"] },
    // { title: "Build from Rust's crates.io", lines: ["cargo install hncli"] },
    // { title: "Install with Homebrew", lines: ["brew install hncli"] },
  ];
</script>

<section id="install" aria-label="Install hncli">
  <div class="website-container">
    <div class="text-center">
      <h2 class="title">How to use</h2>
      <p class="subtitle">More ways to come, at the very least homebrew.</p>
    </div>
    <div class="options-container">
      {#each Object.entries(options) as [i, { title, lines }]}
        <div class="option-container">
          <h3 class="option-title">{title}:</h3>
          <ClipboardCommandLines {lines} extraClass="md:ml-24" />
        </div>
      {/each}
    </div>
  </div>
</section>

<style lang="postcss">
  #install {
    @apply pb-3 pt-12;
  }

  .title {
    @apply mb-1 text-4xl font-semibold text-hncli-dark-red;
  }
  .subtitle {
    @apply text-xs font-medium text-hncli-dark-red/60;
  }

  .options-container {
    @apply flex w-full flex-col pb-6 pt-3 md:flex-row md:justify-center;
  }
  .option-container {
    @apply flex flex-col items-center justify-around pr-3 pt-2;
    .option-title {
      @apply pb-1 text-center text-lg font-medium text-gray-500;
    }
  }
</style>
