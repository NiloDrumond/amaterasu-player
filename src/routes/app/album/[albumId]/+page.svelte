<script lang="ts">
  import { page } from '$app/stores';
  import Loading from '$lib/components/loading.svelte';
  import { ndClient } from '$lib/navidrome/client';
  import type { NDAlbum } from '$lib/navidrome/types';
  import { onDestroy, onMount } from 'svelte';
  import { rgbToHex } from '$lib/utils/color';
  import ColorThief from 'colorthief';

  const albumId = $page.params.albumId;
  let album: NDAlbum | undefined = undefined;

  let img: HTMLImageElement;
  let dominantColor = '#16161e';

  function getDominantColor() {
    const colorThief = new ColorThief();
    const rgb = colorThief.getColor(img);
    dominantColor = rgbToHex(...rgb);
  }

  let coverArtUrl: string = '';
  onMount(async () => {
    album = await ndClient.getAlbum({
      albumId,
    });
  });

  onDestroy(() => {
    if (img) {
      img.removeEventListener('load', getDominantColor);
    }
  });

  $: if (album) {
    coverArtUrl = ndClient.getCoverArtUrl({ coverArtId: album.id });
  }
  $: if (img) {
    img.crossOrigin = 'anonymous';
    if (img.complete) {
      getDominantColor();
    } else {
      img.addEventListener('load', getDominantColor);
    }
  }
  $: console.log(dominantColor);
</script>

<div
  class="w-full bg-secondary-bg flex flex-col justify-start h-full rounded-xl"
>
  {#if album}
    <div
      class={`w-full flex flex-row p-8 gap-6 bg-gradient-to-b from-[${dominantColor}] to-secondary-bg`}
    >
      <img src={coverArtUrl} alt={album.name} bind:this={img} />
      <div class="flex flex-col">
        <h1>{album.name}</h1>
        <h4>{album.artist}</h4>
      </div>
    </div>
  {:else}
    <Loading />
  {/if}
</div>
