<script lang="ts">
  import Header from '$lib/components/header.svelte';
  import PlayerOverlay from '$lib/components/player/player-overlay.svelte';
  import Sidebar from '$lib/components/sidebar.svelte';
  import { user } from '$lib/stores/auth';
  import { currentSong } from '$lib/stores/player';

  $: if (!$user) {
    user.signOut();
  }
</script>

<div
  class="h-[100vh] w-[100vw] flex flex-col max-w-[100vw] max-h-[100vh] overflow-hidden"
>
  <Header />
  <main class="flex flex-row max-h-full max-w-full flex-1">
    <Sidebar />
    <div id="scrollableContainer" class="p-2 w-full overflow-y-auto max-h-full">
      <section class={'flex justify-center items-center flex-1 max-w-full '}>
        <slot />
      </section>
      <div class={$currentSong ? 'h-40' : 'h-20'} />
    </div>
  </main>
  <PlayerOverlay />
</div>
