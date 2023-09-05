<script lang="ts">
  import SearchInput from '$lib/components/search-input.svelte';
  import SongsTable from '$lib/components/songs-table/songs-table.svelte';
  import { ndClient } from '$lib/navidrome/client';
  import { NDSongListSort, type NDSong } from '$lib/navidrome/types';
  import { onMount } from 'svelte';
  import { songListParams } from './store';
  import Icon from '@iconify/svelte';

  let songs: NDSong[] = [];
  async function fetchSongList() {
    songs = await ndClient.getSongList($songListParams);
  }
  $: {
    $songListParams, fetchSongList();
  }
  onMount(() => {
    fetchSongList();
  });
  const { pagination } = songListParams;
</script>

<div class="flex flex-col w-full items-center">
  <div class="filters-toolbar">
    <SearchInput
      initialValue={$songListParams.title}
      onChange={songListParams.setTitle}
    />
    <button
      class="ghost {$songListParams._sort === NDSongListSort.RANDOM
        ? 'text-crystal'
        : ''}"
      on:click={songListParams.setSortRandom}
    >
      <Icon icon="mingcute:shuffle-2-line" />
      RANDOM
    </button>
    <button
      class="ghost {$songListParams._sort === NDSongListSort.PLAY_COUNT
        ? 'text-crystal'
        : ''}"
      on:click={songListParams.setSortPopular}
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
  {#if songs.length > 0}
    <div class="card py-2">
      <SongsTable
        {songs}
        additionalColumns={['album', 'artist', 'playCount']}
      />
    </div>
  {:else}
    <div class="card !w-max p-8">
      <div class="flex flex-row gap-2 items-center">
        <Icon icon="ph:folder-simple-dashed-bold" width={20} height={20} />

        <h3>No results found</h3>
      </div>
    </div>
  {/if}
</div>
