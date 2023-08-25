<script>
  import PlayerOverlay from '$lib/components/player-overlay.svelte';
  import { user } from '$lib/stores/auth';
  import '../app.css';
  import '@fontsource/inter/400.css';
  import '@fontsource/inter/600.css';
  import '@fontsource/inter/700.css';

  $: user.loadUser();
</script>

<div
  class="h-[100vh] w-[100vw] bg-primary-bg flex items-center justify-center flex-col relative"
>
  <slot />
  <!-- <PlayerOverlay /> -->
</div>
