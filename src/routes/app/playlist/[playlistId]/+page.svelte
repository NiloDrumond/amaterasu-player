<script lang="ts">
  import { page } from '$app/stores';
  import Loading from '$lib/components/loading.svelte';
  import { ndClient } from '$lib/navidrome/client';
  import type { NDPlaylist } from '$lib/navidrome/types';
  import { onMount } from 'svelte';
  import Playlist from './playlist.svelte';

  const playlistId = $page.params.playlistId;
  let playlist: NDPlaylist | undefined = undefined;

  onMount(async () => {
    playlist = await ndClient.getPlaylist({
      playlistId,
    });
  });
</script>

<div class="card">
  {#if playlist}
    <Playlist {playlist} />
  {:else}
    <Loading />
  {/if}
</div>
