<script lang="ts">
	import { goto, invalidateAll } from '$app/navigation';
	import DataTable from '$lib/components/ui/data-table/data-table.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import { playlistsColumns } from '$lib/components/playlists/columns.js';
	import { Dialog } from 'bits-ui';
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
			<h1 class="text-xl font-bold tracking-widest uppercase">PLAYLISTS</h1>
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
				<Dialog.Portal>
					<Dialog.Overlay class="fixed inset-0 z-50 bg-black/50" />
					<Dialog.Content
						class="fixed top-1/2 left-1/2 z-50 w-full max-w-md -translate-x-1/2 -translate-y-1/2 rounded-lg bg-background p-6 shadow-lg"
					>
						<Dialog.Title class="mb-1 text-lg font-semibold">New Playlist</Dialog.Title>
						<Dialog.Description class="mb-4 text-sm text-muted-foreground">
							Give your playlist a name to get started.
						</Dialog.Description>
						<form onsubmit={createPlaylist} class="flex flex-col gap-4">
							<div class="flex flex-col gap-2">
								<Label for="playlist-name">Name</Label>
								<Input
									id="playlist-name"
									bind:value={newPlaylistName}
									placeholder="My Playlist"
									autocomplete="off"
									required
								/>
							</div>
							<div class="flex justify-end gap-2">
								<Dialog.Close>
									{#snippet child({ props })}
										<Button {...props} variant="ghost" type="button">Cancel</Button>
									{/snippet}
								</Dialog.Close>
								<Button type="submit" disabled={submitting || !newPlaylistName.trim()}>
									{submitting ? 'Creating…' : 'Create'}
								</Button>
							</div>
						</form>
					</Dialog.Content>
				</Dialog.Portal>
			</Dialog.Root>
		</div>

		<DataTable
			data={data.playlists ?? []}
			{columns}
			onRowClick={(row) => goto(`/playlists/${row.id}`)}
		/>
	</div>
{/if}
