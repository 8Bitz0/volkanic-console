import type { Config } from 'tailwindcss'

export default {
  content: ['src/**/*.{html,js,ts,svelte}'],
  theme: {
    extend: {
      fontFamily: {
        intro: 'Roboto, sans-serif',
        sans: 'Roboto, sans-serif',
      },
    }
  },
  plugins: [],
} satisfies Config

