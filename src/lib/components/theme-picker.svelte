<script lang="ts">
  import { theme, type Theme } from '$lib/stores/theme';
  import { clickoutside } from '@svelteuidev/composables';
  import Icon from '@iconify/svelte';
  const themeIcons = {
    dark: 'mingcute:moon-line',
    light: 'mingcute:sun-line',
    system: 'mingcute:computer-line',
  };
  function getIcon(theme: Theme) {
    switch (theme) {
      case 'dark': {
        return themeIcons.dark;
      }
      case 'light': {
        return themeIcons.light;
      }
      case 'system': {
        const preferDark = window.matchMedia(
          '(prefers-color-scheme: dark)',
        ).matches;
        return preferDark ? themeIcons.dark : themeIcons.light;
      }
    }
  }
  export let variant: 'ghost' | 'solid' = 'ghost';
  let open = false;
  function handleChange(value: Theme) {
    theme.setTheme(value);
    open = false;
  }
</script>

<div
  class="relative"
  use:clickoutside={{ enabled: open, callback: () => (open = false) }}
>
  <button class={variant} on:click={() => (open = !open)}
    ><Icon width={18} height={18} icon={getIcon($theme)} /></button
  >
  {#if open}
    <div
      class="absolute bg-slate-100 rounded-xl border border-slate-400 -right-6 top-[calc(100%_+_1rem)] overflow-hidden z-20 dark:bg-gray-800 dark:border-gray-950"
    >
      <button
        class="menu-item border-t border-slate-300 dark:border-gray-950"
        on:click={() => handleChange('dark')}
        ><Icon width={18} height={18} icon={themeIcons.dark} />Dark</button
      >
      <button
        class="menu-item border-t border-slate-300 dark:border-gray-950"
        on:click={() => handleChange('light')}
        ><Icon width={18} height={18} icon={themeIcons.light} />Light</button
      >
      <button
        class="menu-item border-t border-slate-300 dark:border-gray-950"
        on:click={() => handleChange('system')}
        ><Icon width={18} height={18} icon={themeIcons.system} />System</button
      >
    </div>
  {/if}
</div>
