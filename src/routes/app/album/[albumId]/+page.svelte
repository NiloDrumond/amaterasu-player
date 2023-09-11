<script lang="ts">
  import { page } from '$app/stores';
  import Loading from '$lib/components/loading.svelte';
  import { ndClient } from '$lib/navidrome/client';
  import type { NDAlbum } from '$lib/navidrome/types';
  import { onMount } from 'svelte';
  import Album from './album.svelte';

  const albumId = $page.params.albumId;
  let album: NDAlbum | undefined = undefined;

  onMount(async () => {
    album = await ndClient.getAlbum({
      albumId,
    });
  });
</script>

<div class="card">
  {#if album}
    <Album {album} />
  {:else}
    <Loading />
  {/if}
</div>
