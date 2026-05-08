<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Field, FieldGroup, FieldLabel } from '$lib/components/ui/field/index.js';
	import { toast } from 'svelte-sonner';
	import ListPlusIcon from '@lucide/svelte/icons/list-plus';
	import type { TrackResponse } from '$lib/bindings/response/track/track-response';
	import type { PlaylistResponse } from '$lib/bindings/response/playlist/playlist-response';
	import {
		getPlaylists,
		createPlaylist,
		addTracksToPlaylist,
	} from '$lib/services/playlist-service';

	let {
		tracks,
		open = $bindable(false),
		onClose,
	}: {
		tracks: TrackResponse[];
		open: boolean;
		onClose: () => void;
	} = $props();

	let playlists = $state<PlaylistResponse[]>([]);
	let loading = $state(false);
	let submitting = $state(false);
	let showNewPlaylistInput = $state(false);
	let newPlaylistName = $state('');

	async function fetchPlaylists() {
		loading = true;
		const { data, error } = await getPlaylists(fetch);
		if (error) {
			toast.error('Failed to load playlists');
		} else if (data) {
			playlists = data;
		}
		loading = false;
	}

	async function addTracksToPlaylistById(playlistId: string): Promise<boolean> {
		submitting = true;
		const { error } = await addTracksToPlaylist(fetch, playlistId, {
			trackIds: tracks.map((t) => t.id),
		});
		if (error) {
			toast.error(error);
			submitting = false;
			return false;
		}
		toast.success('Added to playlist');
		open = false;
		submitting = false;
		return true;
	}

	async function createAndAddToPlaylist(e: SubmitEvent) {
		e.preventDefault();
		if (!newPlaylistName.trim()) return;
		submitting = true;
		const { data: newPlaylist, error: createError } = await createPlaylist(fetch, {
			name: newPlaylistName.trim(),
			filterDefinition: null,
		});
		if (createError || !newPlaylist) {
			toast.error('Failed to create playlist');
			submitting = false;
			return;
		}
		const added = await addTracksToPlaylistById(newPlaylist.id);
		if (added) {
			newPlaylistName = '';
			showNewPlaylistInput = false;
		}
		submitting = false;
	}

	$effect(() => {
		if (open) {
			fetchPlaylists();
		} else {
			showNewPlaylistInput = false;
			newPlaylistName = '';
			onClose();
		}
	});
</script>

<Dialog.Root bind:open>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>Add to Playlist</Dialog.Title>
			<Dialog.Description>Choose a playlist or create a new one.</Dialog.Description>
		</Dialog.Header>

		<div class="flex flex-col gap-2">
			{#if showNewPlaylistInput}
				<form onsubmit={createAndAddToPlaylist} class="rounded-md border p-3">
					<FieldGroup>
						<Field>
							<FieldLabel for="new-playlist-name">Playlist name</FieldLabel>
							<Input
								id="new-playlist-name"
								bind:value={newPlaylistName}
								placeholder="My Playlist"
								autocomplete="off"
								required
							/>
						</Field>
						<div class="flex gap-2">
							<Button type="submit" disabled={submitting || !newPlaylistName.trim()} class="flex-1">
								{submitting ? 'Creating…' : 'Create & Add'}
							</Button>
							<Button
								type="button"
								variant="ghost"
								onclick={() => {
									showNewPlaylistInput = false;
									newPlaylistName = '';
								}}
							>
								Cancel
							</Button>
						</div>
					</FieldGroup>
				</form>
			{:else}
				<Button
					variant="outline"
					class="w-full justify-start gap-3 border-dashed font-normal text-muted-foreground"
					onclick={() => (showNewPlaylistInput = true)}
				>
					<ListPlusIcon class="size-4 shrink-0" />
					Create new playlist
				</Button>
			{/if}

			{#if loading}
				<p class="py-4 text-center text-sm text-muted-foreground">Loading playlists…</p>
			{:else if playlists.length === 0}
				<p class="py-4 text-center text-sm text-muted-foreground">No playlists yet.</p>
			{:else}
				<div class="flex max-h-64 flex-col gap-1 overflow-y-auto">
					{#each playlists as playlist (playlist.id)}
						<button
							type="button"
							disabled={submitting}
							class="flex w-full items-center justify-between rounded-md px-3 py-2 text-left text-sm transition-colors hover:bg-muted disabled:opacity-50"
							onclick={() => addTracksToPlaylistById(playlist.id)}
						>
							<span class="truncate font-medium">{playlist.name}</span>
							<span class="ml-2 shrink-0 text-xs text-muted-foreground">
								{playlist.trackCount} tracks
							</span>
						</button>
					{/each}
				</div>
			{/if}
		</div>

		<Dialog.Footer>
			<Dialog.Close>
				{#snippet child({ props })}
					<Button {...props} variant="ghost" type="button">Cancel</Button>
				{/snippet}
			</Dialog.Close>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
