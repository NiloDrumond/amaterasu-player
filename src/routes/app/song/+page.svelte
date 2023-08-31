<script lang="ts">
  import FiltersToolbar from '$lib/components/filters/filters-toolbar.svelte';
  import SongsTable from '$lib/components/songs-table/songs-table.svelte';
  import { ndClient } from '$lib/navidrome/client';
  import { DEFAULT_SONG_LIST_QUERY } from '$lib/navidrome/constants';
  import type { NDSong } from '$lib/navidrome/types';
  import { onMount } from 'svelte';

  let songs: NDSong[] = [];
  let page = 0;
  onMount(async () => {
    songs = await ndClient.getSongList({
      ...DEFAULT_SONG_LIST_QUERY,
    });
  });
</script>

<div class="flex flex-col w-full">
  <FiltersToolbar
    onSearch={() => null}
    prevPage={page > 0 ? () => (page -= 1) : undefined}
    nextPage={() => (page += 1)}
    {page}
  />
  <div class="card py-2">
    <SongsTable {songs} additionalColumns={['album', 'artist', 'playCount']} />
  </div>
</div>
