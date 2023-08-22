<script lang="ts">
  import { ndClient } from '$lib/navidrome/client';
  import type { NDAlbum } from '$lib/navidrome/types';

  export let album: NDAlbum;
  const coverArtUrl = ndClient.getCoverArtUrl({ coverArtId: album.id });
</script>

<!-- TODO: add responsiveness to width -->
<div class="w-[16.6667%] p-2 h-auto">
  <div
    class="bg-secondary-bg h-full overflow-hidden rounded-xl p-3 hover:bg-crystal-primary hover:bg-opacity-80 transition-all flex flex-col gap-2"
  >
    <div class="flex-1" />
    <a href={`/app/album/${album.id}`}>
      <img
        alt={album.name}
        class="object-contain w-full rounded-xl pb-2 h-auto"
        src={coverArtUrl}
      />
    </a>
    <div class="flex-1" />
    <a href={`/app/album/${album.id}`}>
      <p class="font-bold whitespace-nowrap text-ellipsis overflow-hidden">
        {album.name}
      </p>
    </a>
    <a href={`/app/artist/${album.artistId}`}
      ><p class="whitespace-nowrap text-ellipsis text-sm overflow-hidden">
        {album.artist}
      </p></a
    >
  </div>
</div>
