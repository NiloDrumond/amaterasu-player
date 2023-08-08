<script lang="ts">
  import type { NDAlbum } from '$lib/navidrome/types';

  export let album: NDAlbum;
</script>

<!-- TODO: add responsiveness to width -->
<div class="w-[16.6667%] p-2 h-auto">
  <div
    class="bg-secondary-bg h-full overflow-hidden rounded-xl p-3 hover:bg-purple-500 hover:bg-opacity-80 transition-all flex flex-col gap-2"
  >
    <a href="/">
      <img
        alt={album.name}
        class="object-contain w-full rounded-xl pb-2"
        src="https://picsum.photos/720?image=20"
      />
    </a>
    <a href={`/album/${album.id}`}>
      <p class="font-bold whitespace-nowrap text-ellipsis overflow-hidden">
        {album.name}
      </p>
    </a>
    <a href={`/artist/${album.artistId}`}
      ><p class="whitespace-nowrap text-ellipsis text-sm overflow-hidden">
        {album.artist}
      </p></a
    >
  </div>
</div>
