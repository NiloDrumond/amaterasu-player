<script lang="ts">
  import { page } from '$app/stores';
  import Loading from '$lib/components/loading.svelte';
  import { ndClient } from '$lib/navidrome/client';
  import type { NDAlbum } from '$lib/navidrome/types';
  import { onMount } from 'svelte';
  import { formatDuration, formatFileSize } from '$lib/utils/format';
  import Songs from './songs.svelte';
  import Icon from '@iconify/svelte';

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
  class="w-full h-max border border-slate-300 dark:bg-gray-900 flex flex-col justify-start rounded-xl shadow-xl dark:border-gray-950"
>
  {#if album}
    <div class={`w-full flex flex-row p-8 gap-6 `}>
      <img src={coverArtUrl} alt={album.name} class="shadow-xl" />
      <div class="flex flex-col">
        <h2 class="py-4">{album.name}</h2>
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
        <div class="flex flex-row gap-2 items-center">
          <button
            class="rounded-full p-4 bg-crystal hover:scale-105 hover:-translate-y-1 hover:drop-shadow-[0_0_4px_#eb4b98] dark:hover:drop-shadow-[0_0_10px_#f26df9]"
          >
            <Icon
              icon="mingcute:play-fill"
              class="text-gray-950"
              width={26}
              height={26}
            />
          </button>
          <button class="ghost">
            <Icon
              icon="ph:shuffle-angular-bold"
              width={20}
              height={20}
            />SHUFFLE
          </button>
          <button class="ghost"
            ><Icon width={18} height={18} icon="iconamoon:playlist" />PLAY NEXT
          </button>
          <button class="ghost"
            ><Icon width={18} height={18} icon="ph:list-plus-bold" />PLAY LATER
          </button>
          <button class="ghost"
            ><Icon width={18} height={18} icon="ph:music-notes-plus-bold" />ADD
            TO PLAYLIST
          </button>
        </div>
      </div>
    </div>
    <Songs {albumId} />
  {:else}
    <Loading />
  {/if}
</div>
