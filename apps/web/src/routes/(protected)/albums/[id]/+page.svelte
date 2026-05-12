<script lang="ts">
	import { tracksColumns } from '$lib/components/tracks/columns';
	import DataTable from '$lib/components/ui/data-table/data-table.svelte';
	import { Button } from '$lib/components/ui/button';
	import { getPlayer } from '$lib/player/player.svelte';
	import { formatMilliseconds } from '$lib/utils/date';
	import { shuffle } from '$lib/utils/shuffle';
	import PlayIcon from '@lucide/svelte/icons/play';
	import ShuffleIcon from '@lucide/svelte/icons/shuffle';
	import ListStartIcon from '@lucide/svelte/icons/list-start';
	import ListEndIcon from '@lucide/svelte/icons/list-end';
	import ListPlusIcon from '@lucide/svelte/icons/list-plus';
	import MusicIcon from '@lucide/svelte/icons/music';
	import TagIcon from '@lucide/svelte/icons/tag';
	import PencilIcon from '@lucide/svelte/icons/pencil';
	import AddToPlaylistDialog from '$lib/components/playlists/add-to-playlist-dialog.svelte';
	import TagPickerDialog from '$lib/components/tags/tag-picker-dialog.svelte';
	import { page } from '$app/state';

	let { data } = $props();
	const player = getPlayer();
	const isAdmin = $derived(page.data.user?.role === 'admin');

	let addToPlaylistOpen = $state(false);
	let tagsOpen = $state(false);

	const albumColumns = tracksColumns.filter((col) => col.id !== 'album');

	function play() {
		player.playQueue(data.tracks, 0, { albumId: data.album.id });
	}

	function shufflePlay() {
		player.playQueue(shuffle(data.tracks), 0, { albumId: data.album.id });
	}

	function playNext() {
		player.playNext(data.tracks);
	}

	function playLater() {
		player.playLater(data.tracks);
	}

	const year = $derived(data.album.date ? data.album.date.slice(0, 4) : null);
	const duration = $derived(formatMilliseconds(Number(data.album.totalDurationMs)));
</script>

<div class="flex flex-col gap-6 p-6">
	<div class="flex flex-row items-end gap-6">
		{#if data.album.coverUrl}
			<img
				src={data.album.coverUrl}
				alt={data.album.title}
				class="size-48 shrink-0 rounded-lg object-cover shadow-lg"
			/>
		{:else}
			<div
				class="flex size-48 shrink-0 items-center justify-center rounded-lg bg-muted text-muted-foreground"
			>
				<MusicIcon class="size-16 opacity-30" />
			</div>
		{/if}
		<div class="flex min-w-0 flex-col gap-1">
			<p class="text-xs font-medium tracking-widest text-muted-foreground uppercase">Album</p>
			<h1 class="truncate text-3xl">{data.album.title}</h1>
			{#if data.album.artist}
				<a
					href="/artists/{data.album.artist.id}"
					class="w-fit text-sm text-muted-foreground transition-colors hover:text-foreground"
				>
					{data.album.artist.name}
				</a>
			{/if}
			<p class="text-sm text-muted-foreground">
				{[year, `${String(data.album.trackCount)} tracks`, duration].filter(Boolean).join(' · ')}
			</p>
		</div>
	</div>

	<div class="flex flex-row flex-wrap items-center gap-2">
		<Button onclick={play} class="gap-2">
			<PlayIcon class="size-4" />
			Play
		</Button>
		<Button variant="ghost" onclick={shufflePlay} class="gap-2">
			<ShuffleIcon class="size-4" />
			Shuffle
		</Button>
		<Button variant="ghost" onclick={playNext} class="gap-2">
			<ListStartIcon class="size-4" />
			Play Next
		</Button>
		<Button variant="ghost" onclick={playLater} class="gap-2">
			<ListEndIcon class="size-4" />
			Play Later
		</Button>
		<Button variant="ghost" onclick={() => (addToPlaylistOpen = true)} class="gap-2">
			<ListPlusIcon class="size-4" />
			Add to Playlist
		</Button>
		<Button variant="ghost" onclick={() => (tagsOpen = true)} class="gap-2">
			<TagIcon class="size-4" />
			Edit Tags
		</Button>
		{#if isAdmin}
			<Button
				variant="ghost"
				onclick={() => window.open(`/admin/albums/${data.album.id}`, '_blank', 'noopener')}
				class="gap-2"
			>
				<PencilIcon class="size-4" />
				Edit (admin)
			</Button>
		{/if}
	</div>

	<AddToPlaylistDialog
		tracks={data.tracks}
		bind:open={addToPlaylistOpen}
		onClose={() => (addToPlaylistOpen = false)}
	/>

	<TagPickerDialog entity="album" entityId={data.album.id} bind:open={tagsOpen} />

	<DataTable
		storageKey="album:tracks"
		data={data.tracks}
		columns={albumColumns}
		onRowClick={(_, index) => player.playQueue(data.tracks, index, { albumId: data.album.id })}
	/>
</div>
