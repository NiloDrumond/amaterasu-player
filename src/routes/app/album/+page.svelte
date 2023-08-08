<script lang="ts">
  import { ndClient } from '$lib/navidrome/client';
  import type { NDAlbum } from '$lib/navidrome/types';
  import { auth } from '$lib/stores/auth';
  import { onMount } from 'svelte';
  import Album from './album.svelte';

  let albumList: NDAlbum[] = [];
  onMount(async () => {
    albumList = await ndClient.getAlbumList({});
  });
</script>

<div class="flex flex-wrap max-h-full overflow-auto">
  {#each albumList as album, index}
    <Album {album} />
  {/each}
</div>
