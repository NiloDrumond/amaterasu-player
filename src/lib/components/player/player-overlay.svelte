<script lang="ts">
  import { player } from '$lib/stores/player';
  import PlayerSkeleton from './player-skeleton.svelte';
  import PlayerControls from './player-controls.svelte';
  import { ndClient } from '$lib/navidrome/client';
</script>

{#if $player.currentSong}
  <div
    class="sticky bottom-0 left-0 h-24 bg-slate-200 bg-opacity-90 border-t border-slate-300 w-full grid grid-cols-3 dark:bg-gray-900 dark:bg-opacity-90 dark:border-gray-950 p-2"
  >
    <section
      class="flex justify-start items-center gap-3 max-h-full overflow-y-hidden"
    >
      <img
        src={ndClient.getCoverArtUrl({
          coverArtId: $player.currentSong.albumId,
        })}
        alt={$player.currentSong.album}
        class="h-[calc(100%_-_0.5rem)] w-auto"
      />
      <div class="flex flex-col items-start">
        <h4>{$player.currentSong.title}</h4>
        <p>{$player.currentSong.artist} - {$player.currentSong.album}</p>
      </div>
    </section>
    <PlayerControls />
    <section class="flex justify-end items-center">
      Queue, Options, Favorite?, Volume
    </section>
  </div>
{:else}
  <PlayerSkeleton />
{/if}
