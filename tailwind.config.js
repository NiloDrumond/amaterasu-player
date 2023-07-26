/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        'primary-bg': '#1a1b26',
        'secondary-bg': '#16161e',
        primary: '#c0caf5',
        error: '#db4b4b',
        purple: {
          50: '#eda4dc',
          100: '#e991d5',
          200: '#e67fce',
          300: '#e26dc7',
          400: '#df5ac0',
          500: '#DB48B9',
          600: '#c541a7',
          700: '#af3a94',
          800: '#993282',
          900: '#832b6f',
          950: '#6e245d',
        },
        blue: {
          500: '#7595ff',
        },
        cyan: {
          500: '#47BCFF',
        },
      },
    },
  },
  plugins: [],
};
