<script lang="ts">
  import { ndClient } from '$lib/navidrome/client';
  import type { NDAlbum } from '$lib/navidrome/types';

  export let album: NDAlbum;
  const coverArtUrl = ndClient.getCoverArtUrl({ coverArtId: album.id });
</script>

<!-- TODO: add responsiveness to width -->
<div class="w-[14.28%] p-2 h-auto">
  <div
    class="border border-slate-300 h-full overflow-hidden rounded-xl p-3 hover:bg-slate-200 hover:border-crystal hover:bg-opacity-80 transition-all flex flex-col shadow-sm"
  >
    <div class="flex-1 min-h-0" />
    <a href={`/app/album/${album.id}`}>
      <img
        alt={album.name}
        class="object-contain w-full rounded-xl pb-2 h-auto"
        src={coverArtUrl}
      />
    </a>
    <div class="flex-1" />
    <a href={`/app/album/${album.id}`}>
      <p class="font-bold whitespace-nowrap text-ellipsis overflow-hidden mt-2">
        {album.name}
      </p>
    </a>
    <a href={`/app/artist/${album.artistId}`}
      ><p class="whitespace-nowrap text-ellipsis text-sm overflow-hidden mt-2">
        {album.artist}
      </p></a
    >
  </div>
</div>
