<script lang="ts">
  import ThemePicker from '$lib/components/theme-picker.svelte';
  import { user } from '$lib/stores/auth';
  import Icon from '@iconify/svelte';
  import Logo from './logo.svelte';
</script>

<header class="p-2 flex flex-row border-b border-slate-300 shadow-md gap-2">
  <Logo />
  <div class="flex-1" />
  <ThemePicker />
  <button class="ghost" on:click={user.signOut}>
    <Icon width={18} height={18} icon="mingcute:power-line" />
  </button>
</header>
