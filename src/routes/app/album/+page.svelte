<script lang="ts">
  import { ndClient } from '$lib/navidrome/client';
  import { NDAlbumListSort, type NDAlbum } from '$lib/navidrome/types';
  import Album from './album.svelte';
  import { albumListParams } from './store';
  import { onMount } from 'svelte';
  import SearchInput from '$lib/components/search-input.svelte';
  import Icon from '@iconify/svelte';

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
  const { pagination } = albumListParams;
</script>

<div class="w-full flex flex-col">
  <div class="filters-toolbar">
    <SearchInput
      initialValue={$albumListParams.name}
      onChange={albumListParams.setName}
    />
    <button
      class="ghost {$albumListParams._sort === NDAlbumListSort.RANDOM
        ? 'text-crystal'
        : ''}"
      on:click={albumListParams.setSortRandom}
    >
      <Icon icon="mingcute:shuffle-2-line" />
      RANDOM
    </button>
    <button
      class="ghost {$albumListParams._sort === NDAlbumListSort.PLAY_COUNT
        ? 'text-crystal'
        : ''}"
      on:click={albumListParams.setSortPopular}
    >
      <Icon icon="mingcute:star-fill" />
      POPULAR
    </button>
    <div class="flex-1" />
    <button
      class="solid"
      on:click={pagination.prevPage}
      disabled={!$pagination.hasPrevPage}
    >
      <Icon icon="mingcute:left-fill" />
    </button>
    <p>{$pagination.page + 1}</p>
    <button
      class="solid"
      on:click={pagination.nextPage}
      disabled={!$pagination.hasNextPage}
    >
      <Icon icon="mingcute:right-fill" />
    </button>
  </div>

  <div class="flex flex-wrap max-h-full overflow-auto -mx-2">
    {#each albumList as album (album.id)}
      <Album {album} />
    {/each}
  </div>
</div>
