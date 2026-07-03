<script lang="ts">
	import { goto, invalidateAll } from '$app/navigation';
	import { page } from '$app/state';
	import DataTable from '$lib/components/ui/data-table/data-table.svelte';
	import SearchInput from '$lib/components/filters/search-input.svelte';
	import ShuffleButton from '$lib/components/filters/shuffle-button.svelte';
	import { getPlayer } from '$lib/player/player.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Field, FieldGroup, FieldLabel } from '$lib/components/ui/field/index.js';
	import { playlistsColumns } from '$lib/components/playlists/columns.js';
	import PlaylistRowContextMenu from '$lib/components/playlists/playlist-row-context-menu.svelte';
	import {
		applyRandomSortToUrl,
		applySortToUrl,
		extractSortFromUrl,
		generateSeed,
	} from '$lib/utils/pagination';
	import type { SortDir } from '$lib/bindings/request/common/sort-dir';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import { toast } from 'svelte-sonner';
	import { Icons } from '$lib/components/ui/icons';
	import { invalidatePlaylistsCache } from '$lib/state/playlists-cache.svelte';

	let { data } = $props();
	const player = getPlayer();

	let dialogOpen = $state(false);
	let newPlaylistName = $state('');
	let submitting = $state(false);

	async function createPlaylist(e: SubmitEvent) {
		e.preventDefault();
		if (!newPlaylistName.trim()) return;
		submitting = true;
		try {
			const res = await fetch('/api/playlists', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ name: newPlaylistName.trim(), filterDefinition: null }),
			});
			if (res.ok) {
				invalidatePlaylistsCache();
				toast.success('Playlist created');
				dialogOpen = false;
				newPlaylistName = '';
				await invalidateAll();
			} else {
				toast.error('Failed to create playlist');
			}
		} finally {
			submitting = false;
		}
	}

	const columns = $derived(playlistsColumns(() => invalidateAll()));

	function onSearch(q: string) {
		const url = new URL(page.url);
		if (q) url.searchParams.set('q', q);
		else url.searchParams.delete('q');
		url.searchParams.delete('offset');
		goto(url, { keepFocus: true, noScroll: true });
	}

	function onChangePage(newPage: number) {
		if (!data.playlists) return;
		const url = new URL(page.url);
		url.searchParams.set('offset', (data.playlists.limit * (newPage - 1)).toString());
		goto(url);
	}
</script>

{#if data.error}
	<h1>Error</h1>
	<p>{data.error.message}</p>
{:else}
	<div class="flex flex-col gap-3 p-4">
		<h1 class="tracking-widest uppercase">Playlists</h1>
		<div class="flex flex-row flex-wrap items-center gap-2">
			<SearchInput
				value={page.url.searchParams.get('q') ?? ''}
				onChange={onSearch}
				placeholder="Search playlists…"
			/>
			<ShuffleButton
				active={extractSortFromUrl(page.url).sort === 'random'}
				onShuffle={() => {
					goto(applyRandomSortToUrl(new URL(page.url), generateSeed()), {
						keepFocus: true,
						noScroll: true,
					});
				}}
			/>
			<div class="ml-auto"></div>
			<Dialog.Root
				bind:open={dialogOpen}
				onOpenChange={(open) => {
					if (!open) newPlaylistName = '';
				}}
			>
				<Dialog.Trigger>
					{#snippet child({ props })}
						<Button {...props} class="gap-2">
							<Icons.Add class="size-4" />
							New Playlist
						</Button>
					{/snippet}
				</Dialog.Trigger>
				<Dialog.Content>
					<Dialog.Header>
						<Dialog.Title>New Playlist</Dialog.Title>
						<Dialog.Description>Give your playlist a name to get started.</Dialog.Description>
					</Dialog.Header>
					<form onsubmit={createPlaylist}>
						<FieldGroup>
							<Field>
								<FieldLabel for="playlist-name">Name</FieldLabel>
								<Input
									id="playlist-name"
									bind:value={newPlaylistName}
									placeholder="My Playlist"
									autocomplete="off"
									required
								/>
							</Field>
							<Dialog.Footer>
								<Dialog.Close>
									{#snippet child({ props })}
										<Button {...props} variant="ghost" type="button">Cancel</Button>
									{/snippet}
								</Dialog.Close>
								<Button type="submit" disabled={submitting || !newPlaylistName.trim()}>
									{submitting ? 'Creating…' : 'Create'}
								</Button>
							</Dialog.Footer>
						</FieldGroup>
					</form>
				</Dialog.Content>
			</Dialog.Root>
		</div>

		<DataTable
			storageKey="playlists"
			data={data.playlists?.data ?? []}
			{columns}
			pagination={data.playlists
				? {
						page: data.page,
						totalPages: Math.ceil(Number(data.playlists.total) / data.playlists.limit),
						onChangePage,
					}
				: undefined}
			serverSort={{
				...extractSortFromUrl(page.url),
				onSortChange: (sort: string | null, dir: SortDir) => {
					goto(applySortToUrl(new URL(page.url), sort, dir), {
						keepFocus: true,
						noScroll: true,
					});
				},
			}}
			onRowClick={(row) => goto(`/playlists/${row.id}`)}
			isRowPlaying={(row) => row.id === player.currentOrigin.playlistId}
		>
			{#snippet rowContextMenu({ row, trigger })}
				<PlaylistRowContextMenu id={row.id} onDeleted={() => invalidateAll()} {trigger} />
			{/snippet}
		</DataTable>
	</div>
{/if}
