import { writable } from 'svelte/store';

export type Theme = 'light' | 'dark' | 'system';

function createTheme() {
  const { subscribe, set } = writable<Theme>('system');

  function updateTailwind(theme: Theme) {
    if (theme === 'dark') {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
  }

  function loadLocalStorage() {
    const stored = localStorage.getItem('theme');
    let theme: Theme = 'system';
    if (stored === 'dark') {
      theme = 'dark';
    } else if (stored === 'light') {
      theme = 'light';
    } else {
      theme = 'system';
    }
    set(theme);
    updateTailwind(theme);
  }

  function setTheme(theme: Theme) {
    if (theme === 'system') {
      localStorage.removeItem('theme');
    } else {
      localStorage.setItem('theme', theme);
    }
    set(theme);
    updateTailwind(theme);
  }

  loadLocalStorage();
  return { subscribe, setTheme };
}
export const theme = createTheme();
