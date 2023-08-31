<script lang="ts">
  import SongsTable from '$lib/components/songs-table/songs-table.svelte';
  import { ndClient } from '$lib/navidrome/client';
  import { DEFAULT_ALBUM_SONG_LIST_QUERY } from '$lib/navidrome/constants';
  import type { NDSong } from '$lib/navidrome/types';
  import { onMount } from 'svelte';

  export let albumId: string;
  let songs: NDSong[] = [];
  onMount(async () => {
    songs = await ndClient.getSongList({
      album_id: albumId,
      ...DEFAULT_ALBUM_SONG_LIST_QUERY,
    });
  });
</script>

<div class="pb-4">
  <SongsTable {songs} additionalColumns={['artist', 'playCount']} />
</div>
