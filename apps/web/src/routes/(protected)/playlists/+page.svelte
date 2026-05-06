<script lang="ts">
	import { goto, invalidateAll } from '$app/navigation';
	import DataTable from '$lib/components/ui/data-table/data-table.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Field, FieldGroup, FieldLabel } from '$lib/components/ui/field/index.js';
	import { playlistsColumns } from '$lib/components/playlists/columns.js';
	import PlaylistRowContextMenu from '$lib/components/playlists/playlist-row-context-menu.svelte';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import { toast } from 'svelte-sonner';
	import PlusIcon from '@lucide/svelte/icons/plus';

	let { data } = $props();

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
				body: JSON.stringify({ name: newPlaylistName.trim() }),
			});
			if (res.ok) {
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
</script>

{#if data.error}
	<h1>Error</h1>
	<p>{data.error.message}</p>
{:else}
	<div class="flex flex-col p-4">
		<div class="mb-4 flex items-center justify-between">
			<h1 class="tracking-widest uppercase">Playlists</h1>
			<Dialog.Root
				bind:open={dialogOpen}
				onOpenChange={(open) => {
					if (!open) newPlaylistName = '';
				}}
			>
				<Dialog.Trigger>
					{#snippet child({ props })}
						<Button {...props} class="gap-2">
							<PlusIcon class="size-4" />
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
			data={data.playlists ?? []}
			{columns}
			onRowClick={(row) => goto(`/playlists/${row.id}`)}
		>
			{#snippet rowContextMenu({ row, trigger })}
				<PlaylistRowContextMenu id={row.id} onDeleted={() => invalidateAll()} {trigger} />
			{/snippet}
		</DataTable>
	</div>
{/if}
