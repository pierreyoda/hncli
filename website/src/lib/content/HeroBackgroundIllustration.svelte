<script lang="ts">
  export let id: string;
  export let extraClass: string | undefined;

  $: imageClass = extraClass ?? "";
</script>

<div class={imageClass}>
  <svg viewBox="0 0 1026 1026" fill="none" aria-hidden="true" class="absolute inset-0 h-full w-full animate-spin-slow">
    <path
      d="M1025 513c0 282.77-229.23 512-512 512S1 795.77 1 513 230.23 1 513 1s512 229.23 512 512Z"
      stroke="#D4D4D4"
      stroke-opacity="0.7"
    />
    <path d="M513 1025C230.23 1025 1 795.77 1 513" stroke={`url(#${id}-gradient-1)`} stroke-linecap="round" />
    <defs>
      <linearGradient id={`${id}-gradient-1`} x1="1" y1="513" x2="1" y2="1025" gradientUnits="userSpaceOnUse">
        <stop stop-color="#06b6d4" />
        <stop offset="1" stop-color="#06b6d4" stop-opacity="0" />
      </linearGradient>
    </defs>
  </svg>
  <svg
    viewBox="0 0 1026 1026"
    fill="none"
    aria-hidden="true"
    class="absolute inset-0 h-full w-full animate-spin-reverse-slower"
  >
    <path
      d="M913 513c0 220.914-179.086 400-400 400S113 733.914 113 513s179.086-400 400-400 400 179.086 400 400Z"
      stroke="#D4D4D4"
      stroke-opacity="0.7"
    />
    <path d="M913 513c0 220.914-179.086 400-400 400" stroke={`url(#${id}-gradient-2)`} stroke-linecap="round" />
    <defs>
      <linearGradient id={`${id}-gradient-2`} x1="913" y1="513" x2="913" y2="913" gradientUnits="userSpaceOnUse">
        <stop stop-color="#06b6d4" />
        <stop offset="1" stop-color="#06b6d4" stop-opacity="0" />
      </linearGradient>
    </defs>
  </svg>
</div>
