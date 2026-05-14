<script lang="ts">
	import { goto } from '$app/navigation';
	import * as Command from '$lib/components/ui/command';
	import { getPlayer } from '$lib/player/player.svelte';
	import { getAlbumTracks } from '$lib/services/album-service';
	import { getPlaylistTracks } from '$lib/services/playlist-service';
	import { getTrack } from '$lib/services/track-service';
	import {
		searchPalette,
		type PaletteSearchResponse,
		type SearchEntityType,
		type SearchHit,
	} from '$lib/services/search-service';
	import type { TrackResponse } from '$lib/bindings/response/track/track-response';
	import type { PlaylistTrackResponse } from '$lib/bindings/response/playlist/playlist-track-response';
	import { Kbd } from '../ui/kbd';

	const player = getPlayer();

	let open = $state(false);
	let query = $state('');
	// Currently-highlighted item value, in the format `{kind}:{id}` or
	// `go:{href}` / `see-all:{kind}`.
	let highlighted = $state('');
	let results = $state<PaletteSearchResponse>({
		tracks: [],
		albums: [],
		artists: [],
		playlists: [],
		collections: [],
	});
	let loading = $state(false);

	let debounceTimer: ReturnType<typeof setTimeout> | null = null;
	let inflight: AbortController | null = null;

	// Global Ctrl/Cmd+K hotkey.
	$effect(() => {
		function onKeydown(e: KeyboardEvent) {
			if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'k') {
				e.preventDefault();
				open = !open;
			}
		}
		window.addEventListener('keydown', onKeydown);
		return () => window.removeEventListener('keydown', onKeydown);
	});

	// Debounced fetch on query change.
	$effect(() => {
		const q = query.trim();
		if (debounceTimer !== null) clearTimeout(debounceTimer);
		if (inflight) {
			inflight.abort();
			inflight = null;
		}
		if (q.length === 0) {
			results = { tracks: [], albums: [], artists: [], playlists: [], collections: [] };
			loading = false;
			return;
		}
		loading = true;
		debounceTimer = setTimeout(async () => {
			const controller = new AbortController();
			inflight = controller;
			const res = await searchPalette(fetch, q, 5);
			if (controller.signal.aborted) return;
			if (res.data) results = res.data;
			loading = false;
		}, 150);
	});

	// Reset query when palette closes.
	$effect(() => {
		if (!open) {
			query = '';
			loading = false;
		}
	});

	const allHits = $derived<SearchHit[]>([
		...results.tracks,
		...results.albums,
		...results.artists,
		...results.playlists,
		...results.collections,
	]);
	const totalHits = $derived(allHits.length);

	function findHit(value: string): SearchHit | null {
		for (const h of allHits) {
			if (`${h.kind}:${h.id}` === value) return h;
		}
		return null;
	}

	async function navigateTo(hit: SearchHit) {
		open = false;
		switch (hit.kind) {
			case 'track': {
				const res = await getTrack(fetch, hit.id);
				if (res.data?.album) {
					goto(`/albums/${res.data.album.id}`);
				}
				break;
			}
			case 'album':
				goto(`/albums/${hit.id}`);
				break;
			case 'artist':
				goto(`/artists/${hit.id}`);
				break;
			case 'playlist':
				goto(`/playlists/${hit.id}`);
				break;
			case 'collection':
				goto(`/collections/${hit.id}`);
				break;
		}
	}

	async function playHit(hit: SearchHit) {
		open = false;
		switch (hit.kind) {
			case 'track': {
				const res = await getTrack(fetch, hit.id);
				if (res.data) {
					player.playQueue([res.data], 0, { albumId: res.data.album?.id ?? null });
				}
				break;
			}
			case 'album': {
				const res = await getAlbumTracks(fetch, hit.id);
				if (res.data && res.data.length > 0) {
					player.playQueue(res.data, 0, { albumId: hit.id });
				}
				break;
			}
			case 'playlist': {
				const res = await getPlaylistTracks(fetch, hit.id);
				if (res.data && res.data.length > 0) {
					player.playQueue(res.data.map(asTrackResponse), 0, { playlistId: hit.id });
				}
				break;
			}
			default:
				// Artists and collections have no direct play action — fall back to navigate.
				navigateTo(hit);
		}
	}

	function asTrackResponse(t: PlaylistTrackResponse): TrackResponse {
		const {
			playlistTrackId: _playlistTrackId,
			position: _position,
			addedAt: _addedAt,
			artist,
			album,
			...rest
		} = t;
		return {
			...rest,
			artist: artist ? { id: artist.id, name: artist.name } : null,
			album: album ? { id: album.id, title: album.title, coverUrl: album.coverUrl ?? null } : null,
		} as TrackResponse;
	}

	function onInputKeydown(e: KeyboardEvent) {
		if (e.key !== 'Enter' || !e.shiftKey) return;
		const hit = findHit(highlighted);
		if (!hit) return;
		e.preventDefault();
		e.stopPropagation();
		playHit(hit);
	}

	const destinations: Array<{ label: string; href: string }> = [
		{ label: 'Tracks', href: '/tracks' },
		{ label: 'Albums', href: '/albums' },
		{ label: 'Artists', href: '/artists' },
		{ label: 'Playlists', href: '/playlists' },
		{ label: 'Collections', href: '/collections' },
		{ label: 'Tags', href: '/tags' },
		{ label: 'Admin', href: '/admin' },
	];

	function gotoDestination(href: string) {
		open = false;
		goto(href);
	}

	function gotoSearchPage(type: SearchEntityType) {
		open = false;
		const qs = new URLSearchParams({ q: query, type });
		goto(`/search?${qs.toString()}`);
	}

	function groupLabel(kind: SearchEntityType): string {
		switch (kind) {
			case 'track':
				return 'Tracks';
			case 'album':
				return 'Albums';
			case 'artist':
				return 'Artists';
			case 'playlist':
				return 'Playlists';
			case 'collection':
				return 'Collections';
		}
	}

	const groups: Array<{ kind: SearchEntityType; hits: SearchHit[] }> = $derived([
		{ kind: 'track', hits: results.tracks },
		{ kind: 'album', hits: results.albums },
		{ kind: 'artist', hits: results.artists },
		{ kind: 'playlist', hits: results.playlists },
		{ kind: 'collection', hits: results.collections },
	]);
</script>

<Command.Dialog bind:open bind:value={highlighted} shouldFilter={false} label="Search">
	<Command.Input
		bind:value={query}
		placeholder="Search tracks, albums, artists…"
		onkeydown={onInputKeydown}
	/>
	<Command.List>
		{#if query.trim().length === 0}
			<Command.Group heading="Go to">
				{#each destinations as dest (dest.href)}
					<Command.Item value={`go:${dest.href}`} onSelect={() => gotoDestination(dest.href)}>
						{dest.label}
					</Command.Item>
				{/each}
			</Command.Group>
		{:else}
			{#if loading && totalHits === 0}
				<Command.Loading>Searching…</Command.Loading>
			{:else if totalHits === 0}
				<Command.Empty>No results.</Command.Empty>
			{/if}
			{#each groups as group (group.kind)}
				{#if group.hits.length > 0}
					<Command.Group heading={groupLabel(group.kind)}>
						{#each group.hits as hit (hit.id)}
							<Command.Item value={`${group.kind}:${hit.id}`} onSelect={() => navigateTo(hit)}>
								<div class="flex min-w-0 flex-col">
									<span class="truncate">{hit.title}</span>
									{#if hit.subtitle}
										<span class="truncate text-xs text-muted-foreground">{hit.subtitle}</span>
									{/if}
								</div>
							</Command.Item>
						{/each}
						<Command.Item
							value={`see-all:${group.kind}`}
							onSelect={() => gotoSearchPage(group.kind)}
						>
							<span class="text-xs text-muted-foreground"
								>See all {groupLabel(group.kind).toLowerCase()} results →</span
							>
						</Command.Item>
					</Command.Group>
				{/if}
			{/each}
		{/if}
	</Command.List>
	<div class="border-t px-3 py-2 text-xs text-muted-foreground">
		<Kbd>↵</Kbd> open ·
		<Kbd>⇧ + ↵</Kbd> play ·
		<Kbd>esc</Kbd> close
	</div>
</Command.Dialog>
