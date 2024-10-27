import type { Config } from 'tailwindcss';
import remToPx from 'tailwindcss-rem-to-px';
import tailwindScrollbar from 'tailwind-scrollbar';

export default {
  content: ['src/**/*.{html,js,ts,svelte}'],
  theme: {
    extend: {
      fontFamily: {
        sans: '-apple-system, Inter, Roboto, sans-serif',
        windowTitle: '-apple-system, Segoe UI, sans-serif',
        monospace: 'monospace'
      },
    }
  },
  plugins: [
    remToPx({}),
    tailwindScrollbar({
      nocompatible: true,
      preferredStrategy: 'pseudoelements'
    }),
  ],
} satisfies Config

