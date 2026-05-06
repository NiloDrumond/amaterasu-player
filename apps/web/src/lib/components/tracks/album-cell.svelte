<script lang="ts">
	import { page } from '$app/state';
	import type { TrackAlbumResponse } from '$lib/bindings/response/track/track-album-response';

	type AlbumCellProps = {
		album: TrackAlbumResponse;
	};
	let { album }: AlbumCellProps = $props();

	const base = $derived(page.url.pathname.startsWith('/admin') ? '/admin/albums' : '/albums');
</script>

<a href={`${base}/${album.id}`} class="flex flex-row items-center gap-2">
	<img class="size-8" src={album.coverUrl} alt={album.title} />
	<span class="overflow-hidden text-ellipsis">
		{album.title}
	</span>
</a>
