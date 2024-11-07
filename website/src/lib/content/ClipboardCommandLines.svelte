<script lang="ts">
  import Button from "$lib/Button.svelte";
  import CopyIcon from "$lib/assets/copy.svg";
  import { writeToClipboard } from "../../utils";

  interface ClipboardCommandLinesProps {
    lines: readonly string[];
    extraClass?: string;
  }

  const { lines, extraClass }: ClipboardCommandLinesProps = $props();
  const content = $derived<string>(lines.join("\n"));
  const containerClass = $derived<string>(`container ${extraClass ?? ""}`);
  const onCopy = () => writeToClipboard(lines[0]);
</script>

<div class={containerClass}>
  <p class="lines">
    {content}
  </p>
  <Button
    type="action"
    variant="outline"
    color="gray"
    title="Copy to Clipboard"
    onclick={onCopy}
    extraClass="copy-button"
  >
    <img src={CopyIcon} alt="Copy to Clipboard icon" class="copy-icon" />
  </Button>
</div>

<style lang="postcss">
  .container {
    @apply relative rounded-lg bg-gray-900;

    .lines {
      @apply whitespace-pre-line p-6 text-gray-200;
    }

    .copy-icon {
      @apply h-8 w-8;
    }
  }

  .container :global(.copy-button) {
    @apply absolute right-8 font-mono opacity-50 hover:opacity-100;
    @apply top-[50%] translate-y-[-50%];
  }
</style>
