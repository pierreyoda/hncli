import adapterStatic from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/kit/vite";

/** @type {import("@sveltejs/kit").Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    // since this is a simple setup using Vercel hosting, do not provide any custom config
    // see the documentation: https://github.com/sveltejs/kit/tree/master/packages/adapter-static
    adapter: adapterStatic(),
  },
};

export default config;
