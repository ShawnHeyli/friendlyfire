/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,js,ts}", "*.{html,js,ts}"],
  theme: {
    extend: {},
  },
  plugins: [
    require('daisyui'),
  ],
}

