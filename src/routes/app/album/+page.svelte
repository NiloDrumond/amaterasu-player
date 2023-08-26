<script lang="ts">
  import { ndClient } from '$lib/navidrome/client';
  import type { NDAlbum } from '$lib/navidrome/types';
  import Album from './album.svelte';
  import SearchInput from '$lib/components/search-input.svelte';
  import TextButton from '$lib/components/text-button.svelte';
  import { albumListParams } from './store';
  import { onMount } from 'svelte';

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
  <div
    class="mx-2 mb-2 px-3 py-2 flex flex-row sticky top-0 border border-slate-300 bg-slate-100 shadow-lg rounded-xl items-center dark:bg-gray-900 dark:border-gray-950"
  >
    <SearchInput onChange={albumListParams.setName} />
    <div class="flex-1" />
    <TextButton size={16} icon="mingcute:filter-2-line" label="Filters" />
  </div>

  <div class="flex flex-wrap max-h-full overflow-auto">
    {#each albumList as album (album.id)}
      <Album {album} />
    {/each}
  </div>
</div>
