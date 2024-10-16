import type { Config } from 'tailwindcss'

export default {
  content: ['src/**/*.{html,js,ts,svelte}'],
  theme: {
    extend: {
      fontFamily: {
        sans: 'Inter, Roboto, sans-serif',
      },
    }
  },
  plugins: [],
} satisfies Config

