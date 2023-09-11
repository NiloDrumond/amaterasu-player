<script lang="ts">
  import SongsTable from '$lib/components/songs-table/songs-table.svelte';
  import { ndClient } from '$lib/navidrome/client';
  import type { NDAlbum, NDSong } from '$lib/navidrome/types';
  import { formatDuration, formatFileSize } from '$lib/utils/format';
  import { onMount } from 'svelte';
  import Icon from '@iconify/svelte';
  import { DEFAULT_ALBUM_SONG_LIST_QUERY } from '$lib/navidrome/constants';
  import { playerQueue } from '$lib/stores/player-queue';

  export let album: NDAlbum;
  let coverArtUrl: string = '';
  $: if (album) {
    coverArtUrl = ndClient.getCoverArtUrl({ coverArtId: album.id });
  }
  let songs: NDSong[] = [];
  onMount(async () => {
    songs = await ndClient.getSongList({
      album_id: album.id,
      ...DEFAULT_ALBUM_SONG_LIST_QUERY,
    });
  });

  function handleShuffle() {}
  function handlePlayNext() {
    playerQueue.addToQueue(songs, 'next');
  }
  function handlePlayLater() {
    playerQueue.addToQueue(songs, 'later');
  }
  function handleAddToPlaylist() {}
</script>

<div class="card">
  <div class={`w-full flex flex-row p-8 gap-6 `}>
    <img src={coverArtUrl} alt={album.name} class="shadow-xl" />
    <div class="flex flex-col">
      <h2 class="py-4">{album.name}</h2>
      <h4>{album.artist}</h4>
      <div class="flex flex-row gap-2">
        {#if album.minYear !== album.maxYear}
          <p>{album.minYear}-{album.maxYear}</p>
        {:else}
          <p>{album.minYear}</p>
        {/if}
        ·
        <p>{album.songCount} Songs</p>
        ·
        <p>
          {formatDuration(album.duration)}
        </p>
        ·
        <p>
          {formatFileSize(album.size)}
        </p>
        {#if album.playCount > 0}
          ·
          <p>Played {album.playCount} times</p>
        {/if}
      </div>
      <div class="flex-1" />
      <div class="flex flex-row gap-2 items-center">
        <button
          class="rounded-full p-4 bg-crystal hover:scale-105 hover:-translate-y-1 hover:drop-shadow-[0_0_4px_#eb4b98] dark:hover:drop-shadow-[0_0_10px_#f26df9]"
        >
          <Icon
            icon="mingcute:play-fill"
            class="text-gray-950"
            width={26}
            height={26}
          />
        </button>
        <button class="ghost" on:click={handleShuffle}>
          <Icon icon="ph:shuffle-angular-bold" width={20} height={20} />SHUFFLE
        </button>
        <button class="ghost" on:click={handlePlayNext}
          ><Icon width={18} height={18} icon="iconamoon:playlist" />PLAY NEXT
        </button>
        <button class="ghost" on:click={handlePlayLater}
          ><Icon width={18} height={18} icon="ph:list-plus-bold" />PLAY LATER
        </button>
        <button class="ghost" on:click={handleAddToPlaylist}
          ><Icon width={18} height={18} icon="ph:music-notes-plus-bold" />ADD TO
          PLAYLIST
        </button>
      </div>
    </div>
  </div>
  <div class="pb-4">
    <SongsTable
      {songs}
      additionalColumns={['artist', 'playCount', 'trackNumber']}
    />
  </div>
</div>
