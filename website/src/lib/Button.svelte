<script lang="ts">
  type ButtonVariant = "solid" | "outline";
  type ButtonColor = "red" | "white" | "gray";

  const baseStyles: Record<ButtonVariant, string> = {
    solid:
      "inline-flex justify-center rounded-lg py-2 px-3 text-sm font-semibold outline-2 outline-offset-2 transition-colors",
    outline:
      "inline-flex justify-center rounded-lg border py-[calc(theme(spacing.2)-1px)] px-[calc(theme(spacing.3)-1px)] text-sm outline-2 outline-offset-2 transition-colors",
  };
  const variantStyles: Record<ButtonVariant, Record<ButtonColor, string>> = {
    solid: {
      red: "relative overflow-hidden bg-hncli-dark-red text-white before:absolute before:inset-0 active:before:bg-transparent hover:before:bg-white/10 active:bg-hncli-dark-red active:text-white/80 before:transition-colors",
      white: "bg-white text-cyan-900 hover:bg-white/90 active:bg-white/90 active:text-cyan-900/70",
      gray: "bg-gray-800 text-white hover:bg-gray-900 active:bg-gray-800 active:text-white/80",
    },
    outline: {
      red: "",
      white: "",
      gray: "border-gray-300 text-gray-700 hover:border-gray-400 active:bg-gray-100 active:text-gray-700/80",
    },
  };

  export let variant: ButtonVariant = "solid";
  export let color: ButtonColor | undefined;
  export let externalLink: boolean | undefined;
  export let href: string | undefined;
  export let extraClass = "";

  $: buttonColor = variant === "outline" ? "gray" : (color ?? "gray");
  $: buttonClass = `${baseStyles[variant]} ${variantStyles[variant][buttonColor]} ${extraClass}`;
</script>

{#if href}
  <a
    {href}
    class={buttonClass}
    target={externalLink ? "_blank" : undefined}
    rel={externalLink ? "noopener noreferrer" : undefined}
  >
    <slot />
  </a>
{:else}
  <button class={buttonClass}>
    <slot />
  </button>
{/if}
