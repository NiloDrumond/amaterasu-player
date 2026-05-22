<script lang="ts">
	import CarouselSection from '$lib/components/home/carousel-section.svelte';
	import HomeCard from '$lib/components/home/home-card.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Icons } from '$lib/components/ui/icons';
	import { unpinPlaylist } from '$lib/services/pin-service';
	import { invalidateAll } from '$app/navigation';
	import { toast } from 'svelte-sonner';
	import type { HomeItemResponse } from '$lib/bindings/response/me/home-item-response';

	let { data } = $props();

	function itemHref(item: HomeItemResponse): string {
		return item.kind === 'album' ? `/albums/${item.data.id}` : `/playlists/${item.data.id}`;
	}

	function itemTitle(item: HomeItemResponse): string {
		return item.kind === 'album' ? item.data.title : item.data.name;
	}

	function itemSubtitle(item: HomeItemResponse): string | null | undefined {
		return item.kind === 'album'
			? (item.data.artist?.name ?? null)
			: `${item.data.trackCount} tracks`;
	}

	function itemCoverUrl(item: HomeItemResponse): string | null {
		return item.kind === 'album' ? item.data.coverUrl : null;
	}

	function itemFallbackIcon(item: HomeItemResponse): 'music' | 'playlist' {
		return item.kind === 'album' ? 'music' : 'playlist';
	}

	async function handleUnpin(id: string) {
		const { error } = await unpinPlaylist(fetch, id);
		if (error) {
			toast.error(error);
			return;
		}
		await invalidateAll();
	}
</script>

<div class="flex flex-col gap-8 px-6 py-6">
	<CarouselSection
		title="Pinned playlists"
		isEmpty={data.pinnedPlaylists.length === 0}
		emptyMessage="Pin a playlist from its menu to find it here."
	>
		{#each data.pinnedPlaylists as playlist (playlist.id)}
			<HomeCard
				title={playlist.name}
				subtitle="{playlist.trackCount} tracks"
				href="/playlists/{playlist.id}"
				fallbackIcon="playlist"
			>
				{#snippet overlay()}
					<Button
						variant="secondary"
						size="icon"
						class="size-7"
						aria-label="Unpin"
						onclick={(e) => {
							e.preventDefault();
							handleUnpin(playlist.id);
						}}
					>
						<Icons.Close />
					</Button>
				{/snippet}
			</HomeCard>
		{/each}
	</CarouselSection>

	<CarouselSection
		title="Listen again"
		isEmpty={data.listenAgain.length === 0}
		emptyMessage="Play an album or playlist and it'll show up here."
	>
		{#each data.listenAgain as item (item.kind + '-' + item.data.id)}
			<HomeCard
				title={itemTitle(item)}
				subtitle={itemSubtitle(item)}
				coverUrl={itemCoverUrl(item)}
				href={itemHref(item)}
				fallbackIcon={itemFallbackIcon(item)}
			/>
		{/each}
	</CarouselSection>

	<CarouselSection
		title="Forgotten favorites"
		isEmpty={data.forgottenFavorites.length === 0}
		emptyMessage="Once you've played albums or playlists a few times and moved on, they'll resurface here."
	>
		{#each data.forgottenFavorites as item (item.kind + '-' + item.data.id)}
			<HomeCard
				title={itemTitle(item)}
				subtitle={itemSubtitle(item)}
				coverUrl={itemCoverUrl(item)}
				href={itemHref(item)}
				fallbackIcon={itemFallbackIcon(item)}
			/>
		{/each}
	</CarouselSection>

	<CarouselSection
		title="Recently added"
		seeAllHref="/albums?sort=recent&dir=desc"
		isEmpty={data.recentlyAdded.length === 0}
	>
		{#each data.recentlyAdded as album (album.id)}
			<HomeCard
				title={album.title}
				subtitle={album.artist?.name}
				coverUrl={album.coverUrl}
				href="/albums/{album.id}"
			/>
		{/each}
	</CarouselSection>
</div>
