<script>
  import { user } from '$lib/stores/auth';
  import '../app.css';
  import '@fontsource/inter/400.css';
  import '@fontsource/inter/600.css';
  import '@fontsource/inter/700.css';
  import '@fontsource/arvo/400.css';
  import '@fontsource/arvo/700.css';
  import { currentSong } from '$lib/stores/player-queue';

  $: user.loadUser();
</script>

<svelte:head>
  {#if $currentSong}
    <title>{$currentSong.title}</title>
  {:else}
    <title>Amaterasu</title>
  {/if}
</svelte:head>
<div
  class="h-[100vh] w-[100vw] flex items-center justify-center flex-col relative bg-slate-100 dark:bg-gray-800"
>
  <slot />
</div>
