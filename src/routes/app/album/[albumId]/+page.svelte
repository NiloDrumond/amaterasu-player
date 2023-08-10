<script lang="ts">
  import { page } from '$app/stores';
  import Loading from '$lib/components/loading.svelte';
  import { ndClient } from '$lib/navidrome/client';
  import type { NDAlbum } from '$lib/navidrome/types';
  import { onMount } from 'svelte';
  const albumId = $page.params.albumId;

  let album: NDAlbum | undefined = undefined;

  let coverArtUrl: string = '';
  onMount(async () => {
    album = await ndClient.getAlbum({
      albumId,
    });
  });
  $: if (album) {
    coverArtUrl = ndClient.getCoverArtUrl({ coverArtId: album.id });
  }
</script>

<div
  class="w-full bg-secondary-bg flex flex-col justify-start h-full rounded-xl"
>
  {#if album}
    <div class="w-full flex flex-row p-8 gap-6">
      <img src={coverArtUrl} alt={album.name} />
      <div class="flex flex-col">
        <h1>{album.name}</h1>
        <h4>{album.artist}</h4>
      </div>
    </div>
  {:else}
    <Loading />
  {/if}
</div>
