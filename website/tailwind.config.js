/** @type {import("tailwindcss").Config} */
export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    extend: {
      colors: {
        hncli: {
          "dark-red": "#5e1914",
        },
      },
    },
  },
  plugins: [],
};
