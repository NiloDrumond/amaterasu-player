/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  safelist: ['bg-wallpaper-1', 'bg-wallpaper-2'],
  theme: {
    extend: {
      backgroundImage: {
        'wallpaper-1': "url('/wallpapers/wallpaper-1.jpg')",
        'wallpaper-2': "url('/wallpapers/wallpaper-2.jpg')",
      },
      colors: {
        'crystal-primary': '#f26df9',
        'crystal-secondary': '#eb4b98',
        'primary-bg': '#1a1b26',
        'secondary-bg': '#16161e',
        primary: '#c0caf5',
        error: '#db4b4b',
        gray: {
          50: '#5a5b66', // This is a lighter shade of your provided color
          100: '#4a4b56', // Slightly darker
          200: '#3a3b46', // ...
          300: '#2a2b36', // ...
          400: '#23243e', // Almost halfway between the two provided colors
          500: '#1a1b26', // Your provided color
          600: '#18191e', // Slightly darker than your provided color
          700: '#16161e', // Your other provided color
          800: '#12131a', // Slightly darker than your other provided color
          900: '#0e0f16', // The darkest shade of your provided color
          950: '#0a0b12', // Even darker shade, just extending a bit more
        },
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
        green1: {
          500: '#9ece6a',
        },
        green2: {
          500: '#73daca',
        },
        green3: {
          500: '#41a6b5',
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
