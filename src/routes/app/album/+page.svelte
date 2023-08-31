<script lang="ts">
  import { ndClient } from '$lib/navidrome/client';
  import type { NDAlbum } from '$lib/navidrome/types';
  import Album from './album.svelte';
  import { albumListParams } from './store';
  import { onMount } from 'svelte';
  import FiltersToolbar from '$lib/components/filters/filters-toolbar.svelte';

  let albumList: NDAlbum[] = [];
  async function fetchAlbumList() {
    albumList = await ndClient.getAlbumList($albumListParams);
  }
  $: {
    $albumListParams, fetchAlbumList();
  }
  onMount(() => {
    fetchAlbumList();
  });
</script>

<div class="w-full flex flex-col">
  <FiltersToolbar onSearch={albumListParams.setName} />

  <div class="flex flex-wrap max-h-full overflow-auto -mx-2">
    {#each albumList as album (album.id)}
      <Album {album} />
    {/each}
  </div>
</div>
