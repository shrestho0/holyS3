/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {

    extend: {
      fontFamily: {
        custom: ["IBMPlexMono"]
      }

    }
  },
  plugins: [require("daisyui"),],
  daisyui: {
    themes: ["light"], // true: all themes | false: only light + dark | array: specific themes like this ["light", "dark", "cupcake"]

  },
};