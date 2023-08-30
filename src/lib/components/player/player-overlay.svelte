<script lang="ts">
  import {
    currentSong,
    playerQueue,
    currentStreamUrl,
  } from '$lib/stores/player-queue';
  import PlayerControls from './player-controls.svelte';
  import { ndClient } from '$lib/navidrome/client';
  import PlayerOptions from './player-options.svelte';
  import type { EventHandler } from 'svelte/elements';
  import { audioPlayer } from '$lib/stores/audio';

  let isDraggingTimeline = false;
  function handleDraggingTimeline(e: CustomEvent<boolean>) {
    isDraggingTimeline = e.detail;
  }
  function handleEnded(
    e: Parameters<EventHandler<Event, HTMLAudioElement>>[0],
  ) {
    // NOTE: This needs to be manual otherwise it will fail because of "no auto-play policy"
    const song = playerQueue.nextSong();
    if (song) {
      e.currentTarget.src = ndClient.getSongStreamUrl({ songId: song.id });
      e.currentTarget.play();
    }
  }
</script>

<audio
  src={$currentStreamUrl}
  bind:paused={$audioPlayer.paused}
  bind:currentTime={$audioPlayer.currentTime}
  bind:muted={$audioPlayer.muted}
  bind:volume={$audioPlayer.volume}
  on:loadstart={() => {
    $audioPlayer.duration = 0;
  }}
  on:canplaythrough={(e) => {
    if (!$audioPlayer.paused) {
      e.currentTarget.play();
    }
  }}
  on:loadedmetadata={(e) => {
    $audioPlayer.duration = e.currentTarget.duration;
  }}
  on:ended={handleEnded}
/>
{#if $currentSong}
  <div
    class="sticky bottom-0 gap-4 left-0 h-24 transparent border-t w-full grid grid-cols-3 p-2"
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
    <PlayerControls
      on:draggingtimeline={handleDraggingTimeline}
      bind:isDragging={isDraggingTimeline}
    />
    <PlayerOptions />
  </div>
{/if}
