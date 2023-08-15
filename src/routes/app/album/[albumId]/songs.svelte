<script lang="ts">
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
  $: console.log(songs[0]);
</script>

<div>
  <table class="w-full">
    <thead>
      <tr>
        <th><input type="checkbox" /></th>
        <th>#</th>
      </tr>
    </thead>
  </table>
  {#each songs as song}
    <p>{song.title}</p>
  {/each}
</div>
