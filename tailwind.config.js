/** @type {import('tailwindcss').Config} */
export default {
  darkMode: 'class',
  content: ['./src/**/*.{html,js,svelte,ts}'],
  safelist: ['bg-wallpaper-1', 'bg-wallpaper-2'],
  theme: {
    extend: {
      backgroundImage: {
        'wallpaper-1': "url('/wallpapers/wallpaper-1.jpg')",
        'wallpaper-2': "url('/wallpapers/wallpaper-2.jpg')",
      },
      colors: ({ colors }) => ({
        'crystal-primary': '#f26df9',
        'crystal-secondary': '#eb4b98',
        // 'crystal-secondary': '#C039A9',
        crystal: 'var(--color-crystal)',
        text: 'var(--color-text)',
      }),
      fontFamily: {
        title: ['Arvo'],
      },
    },
  },
  plugins: [],
};
