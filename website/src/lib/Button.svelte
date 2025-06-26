<script lang="ts">
  import type { Snippet } from "svelte";

  type ButtonVariant = "solid" | "outline";
  type ButtonColor = "red" | "white" | "gray";

  // TODO: refactor and shorten when tailwindcss@v4 migration is fully done
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

  type ButtonProps = {
    title?: string;
    ariaLabel?: string;
    extraClass?: string;
    children: Snippet;
  } & (
    | {
        type: "link";
        externalLink: boolean;
        href: string;
      }
    | {
        type: "action";
        onclick: () => void;
      }
  ) &
    (
      | {
          variant: "solid";
          color: "red" | "white" | "gray";
        }
      | {
          variant: "outline";
          color: "gray";
        }
    );

  const { variant, color, title, ariaLabel, extraClass, children, ...propsRest }: ButtonProps = $props();

  const buttonColor = $derived<ButtonColor>(variant === "outline" ? "gray" : (color ?? "gray"));
  const buttonClass = $derived<string>(`${baseStyles[variant]} ${variantStyles[variant][buttonColor]} ${extraClass}`);
</script>

{#if propsRest.type === "link"}
  <a
    href={propsRest.href}
    class={buttonClass}
    target={propsRest.externalLink ? "_blank" : undefined}
    rel={propsRest.externalLink ? "noopener noreferrer" : undefined}
    {title}
    aria-label={ariaLabel}
  >
    {@render children()}
  </a>
{:else}
  <button class={buttonClass} aria-label={ariaLabel} onclick={propsRest.onclick} {title}>
    {@render children()}
  </button>
{/if}
