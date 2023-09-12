<script lang="ts">
  import { ndClient } from '$lib/navidrome/client';
  import { type NDPlaylist, NDPlaylistListSort } from '$lib/navidrome/types';
  import Icon from '@iconify/svelte';
  import { onMount } from 'svelte';

  let playlists: NDPlaylist[] = [];

  async function fetchPlaylistList() {
    playlists = await ndClient.getPlaylistList({
      _end: 0,
      _sort: NDPlaylistListSort.NAME,
    });
  }
  onMount(() => {
    fetchPlaylistList();
  });
</script>

<section class="flex flex-col w-full">
  <a href="/app/playlist" class="w-full">
    <button class="menu-item">
      <Icon icon="mingcute:playlist-2-fill" width={20} height={20} />Playlists
    </button>
  </a>
  {#each playlists as playlist}
    <a href="/app/playlist/{playlist.id}">
      <button class="menu-item !py-2 text-sm">
        {playlist.name}
      </button>
    </a>
  {/each}
</section>
