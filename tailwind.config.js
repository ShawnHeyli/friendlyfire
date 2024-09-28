/** @type {import('tailwindcss').Config} */

export default {
  content: ["./src/**/*.{html,js,ts}", "*.{html,js,ts}"],
  theme: {
    extend: {},
  },
  plugins: [
    require('daisyui'),
  ],
  daisyui: {
    themes: [
      "dim",
      {
        // https://www.happyhues.co/palettes/10
        happyhues: {
          "primary": "#f9bc60",
          "secondary": "#abd1c6",
          "accent": "#e16162",
          "neutral": "#abd1c6",
          "base-100": "#004643",
          "info": "#00CED1",
          "success": "#00FA9A",
          "warning": "#FFD700",
          "error": "#B22222",
        },
      },
    ],
  },
}

