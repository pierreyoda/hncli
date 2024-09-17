<script lang="ts">
  import CommentsScreenshot from "$lib/assets/comments.png";
  import ContextualHelpScreenshot from "$lib/assets/contextual-help.png";
  import SettingsScreenshot from "$lib/assets/settings.png";
  import ChatBubbleLeftRightIcon from "$lib/assets/icons/chat-bubble-left-right.svg";
  import QuestionMarkCircleIcon from "$lib/assets/icons/question-mark-circle.svg";
  import Cog8ToothIcon from "$lib/assets/icons/cog-8-tooth.svg";

  const featureSections = ["Comments", "ContextualHelp", "Settings"] as const;
  type FeatureSection = (typeof featureSections)[number];
  type FeatureMeta = {
    title: string;
    description: string;
    icon: string;
    screenshot: string;
  };
  const featuresMeta: Record<FeatureSection, FeatureMeta> = {
    Comments: {
      title: "Browse through every comment",
      description: "However deep a comment is, you can browse to and from it easily.",
      icon: ChatBubbleLeftRightIcon,
      screenshot: CommentsScreenshot,
    },
    ContextualHelp: {
      title: "Pick up topics from where you last read its comments", // TODO: better title
      description:
        "hncli will remember what comment on a story you were last reading, and by default will restore your view to it.",
      icon: QuestionMarkCircleIcon,
      screenshot: ContextualHelpScreenshot,
    },
    Settings: {
      title: "Customize your experience",
      description: "Persistent settings, stored in the OS-appropriate folder.",
      icon: Cog8ToothIcon,
      screenshot: SettingsScreenshot,
    },
  };

  let currentSection: FeatureSection = "Comments";
  const changeSection = (section: string) => (currentSection = section as FeatureSection);
  $: currentScreenshot = featuresMeta[currentSection].screenshot;
</script>

<section aria-label="hncli main features" id="features" class="bg-gray-900 py-20 sm:py-32">
  <div class="website-container">
    <div class="mx-auto max-w-2xl lg:mx-0 lg:max-w-3xl">
      <h2 class="text-3xl font-medium tracking-tight text-white">Features</h2>
      <p class="mt-2 text-lg text-gray-400">
        With a read-only focus and a sleek UX, you will have everything you need to browser Hacker News.
      </p>
    </div>
  </div>
  <div class="website-container mx-auto mt-16 md:mt-20 flex flex-col md:flex-row md:items-center">
    <div class="w-1/2">
      <img alt="hncli feature screenshot" src={currentScreenshot} class="mx-auto max-w-[42rem] h-auto" />
    </div>
    <div class="w-1/2 flex flex-row md:flex-col items-center">
      {#each Object.entries(featuresMeta) as [sectionId, { title, description, icon, screenshot }]}
        <button on:click={() => changeSection(sectionId)} class="rounded-2xl transition-colors hover:bg-gray-800/20">
          <div class="p-8 flex flex-col items-center">
            <img src={icon} alt="hncli feature screenshot" class="h-8 w-8 text-white" />
            <h3 class="mt-6 text-lg font-semibold text-white">
              {title}
            </h3>
            <p class="mt-2 text-sm text-gray-400">
              {description}
            </p>
          </div>
        </button>
      {/each}
    </div>
  </div>
</section>
