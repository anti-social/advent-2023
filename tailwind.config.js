/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: [
    "./src/**/*.{rs,html,css}",
    "./dist/**/*.html",
  ],
  safelist: [
    "hljs-ln-numbers",
    "hljs-ln-code",
  ],
  theme: {
    container: {
      center: true,
    },
    extend: {},
  },
  plugins: [],
}

