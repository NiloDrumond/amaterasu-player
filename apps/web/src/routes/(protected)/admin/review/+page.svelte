<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import TrackEditForm from '$lib/components/admin/track-edit-form.svelte';
	import AlbumEditForm from '$lib/components/admin/album-edit-form.svelte';
	import ArtistEditForm from '$lib/components/admin/artist-edit-form.svelte';
	import { approveAlbumCascade } from '$lib/services/admin-service';
	import { toast } from 'svelte-sonner';
	import { invalidateAll, goto } from '$app/navigation';
	import type { ReviewQueueAlbumGroup } from '$lib/bindings/response/admin/review-queue-album-group';

	let { data } = $props();

	let expanded = $state<Record<string, boolean>>({});
	let artistExpanded = $state<Record<string, boolean>>({});
	let showApproved = $state<Record<string, boolean>>({});
	let cascading = $state<Record<string, boolean>>({});

	function pendingCount(group: ReviewQueueAlbumGroup): number {
		let n = 0;
		if (!group.album.approved) n++;
		if (group.artist && !group.artist.approved) n++;
		for (const t of group.tracks) if (!t.approved) n++;
		return n;
	}

	async function cascadeApprove(albumId: string) {
		cascading[albumId] = true;
		const { error } = await approveAlbumCascade(fetch, albumId);
		cascading[albumId] = false;
		if (error) toast.error('Approve failed', { description: error });
		else {
			toast.success('Album and tracks approved');
			await invalidateAll();
		}
	}

	function gotoOffset(newOffset: number) {
		const qs = newOffset <= 0 ? '' : `?offset=${newOffset}`;
		goto(`/admin/review${qs}`, { keepFocus: true, noScroll: false });
	}

	let hasNextPage = $derived(data.queue.albums.length >= data.pageSize);
</script>

<div class="space-y-6 p-6">
	<header class="mx-auto max-w-4xl space-y-1">
		<h1>Review queue</h1>
		<p class="text-sm text-muted-foreground">
			{data.queue.counts.pendingAlbums} album{data.queue.counts.pendingAlbums === 1n ? '' : 's'} ·
			{data.queue.counts.pendingTracks} track{data.queue.counts.pendingTracks === 1n ? '' : 's'} ·
			{data.queue.counts.pendingArtists}
			artist{data.queue.counts.pendingArtists === 1n ? '' : 's'} pending
		</p>
		<p class="text-xs text-muted-foreground">
			Approve entities individually, or use “Approve album + all tracks” to clear a whole group.
		</p>
	</header>

	{#if data.queue.standaloneArtists.length > 0}
		<section class="mx-auto max-w-4xl space-y-3">
			<h2 class="text-lg font-semibold">Pending artists ({data.queue.standaloneArtists.length})</h2>
			<p class="text-xs text-muted-foreground">
				These artists have no pending albums on this page. Approve or delete them directly.
			</p>
			<div class="space-y-4">
				{#each data.queue.standaloneArtists as artist (artist.id)}
					{@const isExpanded = artistExpanded[artist.id] === true}
					<article class="rounded-lg border bg-card shadow-sm">
						<header class="flex flex-wrap items-baseline gap-3 p-4">
							<div class="min-w-0 flex-1">
								<h3 class="truncate text-base font-semibold">{artist.name}</h3>
								<p class="truncate text-xs text-muted-foreground">{artist.id}</p>
							</div>
							<span
								class="rounded-md bg-amber-500/10 px-2 py-1 text-xs font-medium text-amber-600 dark:text-amber-400"
							>
								Pending
							</span>
							<Button
								variant="ghost"
								size="sm"
								onclick={() => (artistExpanded[artist.id] = !isExpanded)}
							>
								{isExpanded ? 'Collapse' : 'Expand'}
							</Button>
						</header>
						{#if isExpanded}
							<div class="border-t p-4">
								<ArtistEditForm
									{artist}
									canDelete={true}
									onAfterChange={() => invalidateAll()}
									onAfterDelete={() => invalidateAll()}
								/>
							</div>
						{/if}
					</article>
				{/each}
			</div>
		</section>
	{/if}

	<section class="mx-auto max-w-4xl space-y-4">
		<h2 class="text-lg font-semibold">Albums with pending content ({data.queue.albums.length})</h2>

		{#if data.queue.albums.length === 0 && data.queue.standaloneArtists.length === 0}
			<p class="rounded-md border bg-muted/30 px-4 py-8 text-center text-sm text-muted-foreground">
				Nothing to review. New imports and edits will show up here.
			</p>
		{/if}

		{#each data.queue.albums as group (group.album.id)}
			{@const pendingTracks = group.tracks.filter((t) => !t.approved)}
			{@const approvedTracks = group.tracks.filter((t) => t.approved)}
			{@const isExpanded = expanded[group.album.id] === true}
			<article class="rounded-lg border bg-card shadow-sm">
				<header class="flex flex-wrap items-baseline gap-3 border-b p-4">
					<div class="min-w-0 flex-1">
						<h3 class="truncate text-base font-semibold">{group.album.title}</h3>
						<p class="truncate text-sm text-muted-foreground">
							{group.artist?.name ?? '(no artist)'} · {group.tracks.length} track{group.tracks
								.length === 1
								? ''
								: 's'}
							{#if pendingTracks.length > 0}
								· <span class="text-amber-600 dark:text-amber-400"
									>{pendingTracks.length} pending</span
								>
							{/if}
						</p>
					</div>

					<Button
						onclick={() => cascadeApprove(group.album.id)}
						disabled={cascading[group.album.id] || pendingCount(group) === 0}
					>
						{cascading[group.album.id]
							? 'Approving…'
							: pendingCount(group) === 0
								? 'All approved'
								: 'Approve album + all tracks'}
					</Button>
					<Button
						variant="ghost"
						size="sm"
						onclick={() => (expanded[group.album.id] = !isExpanded)}
					>
						{isExpanded ? 'Collapse' : 'Expand'}
					</Button>
				</header>

				{#if isExpanded}
					<div class="space-y-6 p-4">
						<div class="space-y-2">
							<div class="flex items-baseline justify-between gap-2">
								<h4 class="text-sm font-semibold tracking-wide text-muted-foreground uppercase">
									Album
								</h4>
								{#if !group.album.approved}
									<span
										class="rounded-md bg-amber-500/10 px-2 py-0.5 text-xs font-medium text-amber-600 dark:text-amber-400"
									>
										Pending
									</span>
								{/if}
							</div>
							<AlbumEditForm
								album={group.album}
								artist={group.artist}
								canDelete={group.tracks.length === 0}
								showArtistLink={false}
								onAfterChange={() => invalidateAll()}
								onAfterDelete={() => invalidateAll()}
							/>
						</div>

						{#if group.artist && !group.artist.approved}
							<div class="space-y-2">
								<div class="flex items-baseline justify-between gap-2">
									<h4 class="text-sm font-semibold tracking-wide text-muted-foreground uppercase">
										Artist
									</h4>
									<span
										class="rounded-md bg-amber-500/10 px-2 py-0.5 text-xs font-medium text-amber-600 dark:text-amber-400"
									>
										Pending
									</span>
								</div>
								<ArtistEditForm
									artist={group.artist}
									canDelete={false}
									onAfterChange={() => invalidateAll()}
									onAfterDelete={() => invalidateAll()}
								/>
							</div>
						{/if}

						<div class="space-y-3">
							<h4 class="text-sm font-semibold tracking-wide text-muted-foreground uppercase">
								Pending tracks ({pendingTracks.length})
							</h4>
							{#if pendingTracks.length === 0}
								<p class="text-sm text-muted-foreground">No pending tracks on this album.</p>
							{:else}
								{#each pendingTracks as track (track.id)}
									<div class="rounded-md border bg-background p-3">
										<TrackEditForm
											{track}
											artist={group.artist}
											album={group.album}
											showArtistLink={false}
											showAlbumLink={false}
											showAlbumPicker={false}
											onAfterChange={() => invalidateAll()}
											onAfterDelete={() => invalidateAll()}
										/>
									</div>
								{/each}
							{/if}
						</div>

						{#if approvedTracks.length > 0}
							<div class="space-y-2 border-t pt-3">
								<button
									type="button"
									class="text-xs font-semibold tracking-wide text-muted-foreground uppercase hover:text-foreground"
									onclick={() => (showApproved[group.album.id] = !showApproved[group.album.id])}
								>
									{showApproved[group.album.id] ? '▾' : '▸'} Approved tracks ({approvedTracks.length})
								</button>
								{#if showApproved[group.album.id]}
									<ul class="space-y-1 text-sm">
										{#each approvedTracks as track (track.id)}
											<li class="flex items-center justify-between gap-2 px-1">
												<span class="truncate">
													{#if track.trackNo != null}<span class="text-muted-foreground"
															>{track.trackNo}.</span
														>{/if}
													{track.title}
												</span>
												<a
													href="/admin/tracks/{track.id}"
													class="shrink-0 text-xs text-muted-foreground hover:text-foreground"
													>Edit →</a
												>
											</li>
										{/each}
									</ul>
								{/if}
							</div>
						{/if}
					</div>
				{/if}
			</article>
		{/each}

		{#if data.queue.albums.length > 0}
			<div class="flex items-center justify-between gap-2 pt-2">
				<Button
					variant="outline"
					disabled={data.offset === 0}
					onclick={() => gotoOffset(Math.max(0, data.offset - data.pageSize))}
				>
					← Previous
				</Button>
				<span class="text-xs text-muted-foreground">
					Showing {data.offset + 1}–{data.offset + data.queue.albums.length}
				</span>
				<Button
					variant="outline"
					disabled={!hasNextPage}
					onclick={() => gotoOffset(data.offset + data.pageSize)}
				>
					Next →
				</Button>
			</div>
		{/if}
	</section>
</div>
