<script lang="ts">
  import { page } from '$app/stores';
  import Loading from '$lib/components/loading.svelte';
  import { ndClient } from '$lib/navidrome/client';
  import type { NDAlbum } from '$lib/navidrome/types';
  import { onMount } from 'svelte';
  import { formatDuration, formatFileSize } from '$lib/utils/format';
  import IconButton from '$lib/components/icon-button.svelte';
  import PlayButton from '$lib/components/play-button.svelte';
  import TextButton from '$lib/components/text-button.svelte';
  import Songs from './songs.svelte';

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
  class="w-full h-max bg-secondary-bg flex flex-col justify-start rounded-xl"
>
  {#if album}
    <div class={`w-full flex flex-row p-8 gap-6 `}>
      <img src={coverArtUrl} alt={album.name} class="shadow-xl" />
      <div class="flex flex-col">
        <h1 class="py-4">{album.name}</h1>
        <h4>{album.artist}</h4>
        <div class="flex flex-row gap-2">
          {#if album.minYear !== album.maxYear}
            <p>{album.minYear}-{album.maxYear}</p>
          {:else}
            <p>{album.minYear}</p>
          {/if}
          ·
          <p>{album.songCount} Songs</p>
          ·
          <p>
            {formatDuration(album.duration)}
          </p>
          ·
          <p>
            {formatFileSize(album.size)}
          </p>
          {#if album.playCount > 0}
            ·
            <p>Played {album.playCount} times</p>
          {/if}
        </div>
        <div class="flex-1" />
        <div class="flex flex-row gap-4 items-center">
          <PlayButton icon="mingcute:play-fill" />
          <IconButton icon="ph:shuffle-angular-bold" />
          <TextButton label="PLAY NEXT" icon="iconamoon:playlist" />
          <TextButton label="PLAY LATER" icon="ph:list-plus-bold" />
          <TextButton label="ADD TO PLAYLIST" icon="ph:music-notes-plus-bold" />
        </div>
      </div>
    </div>
    <Songs {albumId} />
  {:else}
    <Loading />
  {/if}
</div>
