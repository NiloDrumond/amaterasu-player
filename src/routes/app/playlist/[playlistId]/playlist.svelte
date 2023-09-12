<script lang="ts">
  import SongsTable from '$lib/components/songs-table/songs-table.svelte';
  import type { NDPlaylist, NDPlaylistSong } from '$lib/navidrome/types';
  import { formatDuration, formatFileSize } from '$lib/utils/format';
  import Icon from '@iconify/svelte';
  import { playerQueue } from '$lib/stores/player-queue';
  import { ndClient } from '$lib/navidrome/client';
  import { onMount } from 'svelte';
  import { DEFAULT_PLAYLIST_SONG_LIST_QUERY } from '$lib/navidrome/constants';

  export let playlist: NDPlaylist;

  let songs: NDPlaylistSong[] = [];
  onMount(async () => {
    songs = await ndClient.getPlaylistSongList({
      playlist_id: playlist.id,
      ...DEFAULT_PLAYLIST_SONG_LIST_QUERY,
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
    <div class="flex flex-col">
      <h2 class="py-4">{playlist.name}</h2>
      <div class="flex flex-row gap-2">
        <p>{playlist.songCount} Songs</p>
        ·
        <p>
          {formatDuration(playlist.duration)}
        </p>
        ·
        <p>
          {formatFileSize(playlist.size)}
        </p>
      </div>
      <div class="flex-1" />
      <div class="flex flex-row gap-2 items-center">
        <button class="play">
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
