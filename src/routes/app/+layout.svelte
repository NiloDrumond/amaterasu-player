<script lang="ts">
  import Header from '$lib/components/header.svelte';
  import PlayQueue from '$lib/components/play-queue.svelte';
  import PlayerOverlay from '$lib/components/player/player-overlay.svelte';
  import Sidebar from '$lib/components/sidebar/sidebar.svelte';
  import { audioPlayer } from '$lib/stores/audio';
  import { user } from '$lib/stores/auth';
  import { currentSong } from '$lib/stores/player-queue';
  import { scan } from '$lib/stores/scan';
  import { showPlayQueue } from '$lib/stores/ui';

  $: if (!$user) {
    user.signOut();
  }
  scan.initialize();

  function onKeyUp(
    event: KeyboardEvent & {
      currentTarget: EventTarget & Window;
    },
  ) {
    const el = event.target as Element;
    // if (['INPUT', 'BUTTON', 'A'].includes(el.tagName)) return;
    if (['INPUT'].includes(el.tagName)) return;
    if (event.code === 'Space') {
      audioPlayer.togglePlaying();
    }
    if (event.code === 'KeyM') {
      audioPlayer.toggleMute();
    }
    event.preventDefault();
    event.stopPropagation();
  }
</script>

<svelte:window on:keyup={onKeyUp} />
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
      <div class={$currentSong ? 'h-40' : 'h-16'} />
    </div>
  </main>
  {#if $showPlayQueue}
    <PlayQueue />
  {/if}
  <PlayerOverlay />
</div>
