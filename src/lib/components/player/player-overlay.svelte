<script lang="ts">
  import {
    currentSong,
    audioPlayer,
    effectiveVolume,
  } from '$lib/stores/player';
  import PlayerControls from './player-controls.svelte';
  import { ndClient } from '$lib/navidrome/client';
  import PlayerOptions from './player-options.svelte';

  let streamUrl: string | undefined;
  $: if ($currentSong) {
    streamUrl = ndClient.getSongStreamUrl({
      songId: $currentSong.id,
    });
  }
</script>

{#if $currentSong}
  <audio
    src={streamUrl}
    bind:paused={$audioPlayer.paused}
    bind:currentTime={$audioPlayer.currentTime}
    volume={$effectiveVolume}
    on:loadstart={() => {
      $audioPlayer.duration = 0;
    }}
    on:loadedmetadata={(e) => {
      $audioPlayer.duration = e.currentTarget.duration;
    }}
    on:ended={() => {
      // TODO: HANDLE DRAG
      // currentTime = 0;
    }}
  />
  <div
    class="sticky bottom-0 gap-4 left-0 h-24 bg-slate-200 bg-opacity-90 border-t border-slate-300 w-full grid grid-cols-3 dark:bg-gray-900 dark:bg-opacity-90 dark:border-gray-950 p-2"
  >
    <section
      class="flex justify-start items-center gap-3 max-h-full overflow-y-hidden"
    >
      <img
        src={ndClient.getCoverArtUrl({
          coverArtId: $currentSong.albumId,
        })}
        alt={$currentSong.album}
        class="h-[calc(100%_-_0.5rem)] w-auto"
      />
      <div class="flex flex-col items-start">
        <h4>{$currentSong.title}</h4>
        <p>{$currentSong.artist} - {$currentSong.album}</p>
      </div>
    </section>
    <PlayerControls />
    <PlayerOptions />
  </div>
{/if}
