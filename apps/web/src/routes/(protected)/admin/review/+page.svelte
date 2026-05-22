<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import TrackEditForm from '$lib/components/admin/track-edit-form.svelte';
	import AlbumEditForm from '$lib/components/admin/album-edit-form.svelte';
	import ArtistEditForm from '$lib/components/admin/artist-edit-form.svelte';
	import MbSuggestionList from '$lib/components/admin/mb-suggestion-list.svelte';
	import {
		approveAlbumCascade,
		approveAlbum,
		approveArtist,
		approveTrack,
		uploadAlbumCover,
	} from '$lib/services/admin-service';
	import {
		bulkLookupPending,
		getAlbumSuggestions,
		getArtistSuggestions,
		lookupAlbum,
		lookupArtist,
	} from '$lib/services/musicbrainz-service';
	import { formatMilliseconds } from '$lib/utils/date';
	import { toast } from 'svelte-sonner';
	import { invalidateAll, goto } from '$app/navigation';
	import type { MetadataSuggestionResponse } from '$lib/bindings/response/admin/metadata-suggestion-response';
	import type { ReviewQueueAlbumGroup } from '$lib/bindings/response/admin/review-queue-album-group';

	let { data } = $props();

	let editing = $state<Record<string, boolean>>({});
	let expanded = $state<Record<string, boolean>>({});
	let showApproved = $state<Record<string, boolean>>({});
	let cascading = $state<Record<string, boolean>>({});
	let approving = $state<Record<string, boolean>>({});
	let mbLooking = $state<Record<string, boolean>>({});
	let mbExpanded = $state<Record<string, boolean>>({});
	let bulkLooking = $state(false);
	let uploadingCover = $state<Record<string, boolean>>({});

	/**
	 * Client-side override of server-loaded suggestions: when polling lands
	 * fresh results after a manual lookup, we write them here and the UI
	 * prefers this map over `data.albumSuggestions` / `data.artistSuggestions`.
	 */
	let liveSuggestions = $state<Record<string, MetadataSuggestionResponse[]>>({});

	function suggestionsFor(kind: 'album' | 'artist', id: string): MetadataSuggestionResponse[] {
		const live = liveSuggestions[id];
		if (live) return live;
		const fromServer = kind === 'album' ? data.albumSuggestions[id] : data.artistSuggestions[id];
		return fromServer ?? [];
	}

	/** Latest `createdAt` across a suggestion list, or empty string. */
	function maxCreatedAt(list: MetadataSuggestionResponse[]): string {
		let max = '';
		for (const s of list) if (s.createdAt > max) max = s.createdAt;
		return max;
	}

	const POLL_INTERVAL_MS = 1500;
	const POLL_MAX_MS = 30_000;

	async function pollForFreshSuggestions(
		kind: 'album' | 'artist',
		id: string,
		baselineCreatedAt: string,
	): Promise<void> {
		const start = Date.now();
		const fetchFn = kind === 'album' ? getAlbumSuggestions : getArtistSuggestions;
		while (Date.now() - start < POLL_MAX_MS) {
			await new Promise((r) => setTimeout(r, POLL_INTERVAL_MS));
			const { data: list } = await fetchFn(fetch, id);
			if (list && list.length > 0 && maxCreatedAt(list) > baselineCreatedAt) {
				liveSuggestions[id] = list;
				mbExpanded[id] = true;
				toast.success(`MusicBrainz: ${list.length} candidate${list.length === 1 ? '' : 's'}`);
				return;
			}
		}
		toast.info('No new MusicBrainz suggestions yet — try again in a moment.');
	}

	async function lookupOnMb(kind: 'album' | 'artist', id: string) {
		mbLooking[id] = true;
		const baseline = maxCreatedAt(suggestionsFor(kind, id));
		const { error } = await (kind === 'album' ? lookupAlbum(fetch, id) : lookupArtist(fetch, id));
		if (error) {
			mbLooking[id] = false;
			toast.error('MusicBrainz lookup failed', { description: error });
			return;
		}
		toast.success('Looking up on MusicBrainz…');
		await pollForFreshSuggestions(kind, id, baseline);
		mbLooking[id] = false;
	}

	async function onCoverFileChange(albumId: string, e: Event) {
		const input = e.target as HTMLInputElement;
		const file = input.files?.[0];
		// Always clear so the same file can be re-selected later.
		input.value = '';
		if (!file) return;
		uploadingCover[albumId] = true;
		const { error } = await uploadAlbumCover(fetch, albumId, file);
		uploadingCover[albumId] = false;
		if (error) {
			toast.error('Cover upload failed', { description: error });
		} else {
			toast.success('Cover updated');
			await invalidateAll();
		}
	}

	async function lookupAllPending() {
		bulkLooking = true;
		const { data: resp, error } = await bulkLookupPending(fetch);
		bulkLooking = false;
		if (error) toast.error('Bulk lookup failed', { description: error });
		else toast.success(`Enqueued ${resp?.enqueued ?? 0n} MusicBrainz lookups`);
	}

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

	async function approveEntity(kind: 'album' | 'artist' | 'track', id: string) {
		approving[id] = true;
		const fn = kind === 'album' ? approveAlbum : kind === 'artist' ? approveArtist : approveTrack;
		const { error } = await fn(fetch, id);
		approving[id] = false;
		if (error) toast.error('Approve failed', { description: error });
		else {
			toast.success('Approved');
			await invalidateAll();
		}
	}

	function toggleEdit(id: string) {
		editing[id] = !editing[id];
	}

	async function afterChange(id: string) {
		editing[id] = false;
		await invalidateAll();
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
			Each row shows the basics — click <strong>Edit</strong> if something looks off, or
			<strong>Approve</strong> to clear it.
		</p>
		<div class="pt-2">
			<Button size="sm" variant="outline" onclick={lookupAllPending} disabled={bulkLooking}>
				{bulkLooking ? 'Enqueueing…' : 'Lookup all pending on MusicBrainz'}
			</Button>
		</div>
	</header>

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
			{@const trackArtistMap = new Map(group.trackArtists.map((a) => [a.id, a]))}
			{@const trackArtistFor = (artistId: string | null) =>
				artistId == null
					? null
					: (trackArtistMap.get(artistId) ?? (group.artist?.id === artistId ? group.artist : null))}
			{@const albumEditing = editing[group.album.id] === true}
			{@const artistEditing = group.artist ? editing[group.artist.id] === true : false}
			{@const isExpanded = expanded[group.album.id] === true}
			{@const albumSuggs = suggestionsFor('album', group.album.id)}
			<article class="overflow-hidden rounded-lg border bg-card shadow-sm">
				<header
					class="flex flex-wrap items-center gap-3 bg-muted/30 px-4 py-3"
					class:border-b={isExpanded}
				>
					<label
						class="group relative size-16 shrink-0 cursor-pointer overflow-hidden rounded border bg-muted"
						title="Click to upload a new cover"
					>
						{#if group.album.coverUrl}
							<img
								src={group.album.coverUrl}
								alt="cover for {group.album.title}"
								class="size-full object-cover"
							/>
						{:else}
							<span
								class="flex size-full items-center justify-center text-[10px] font-medium tracking-wide text-muted-foreground uppercase"
							>
								No cover
							</span>
						{/if}
						<span
							class="absolute inset-0 flex items-center justify-center bg-black/40 text-[10px] font-semibold text-white opacity-0 transition group-hover:opacity-100"
						>
							{uploadingCover[group.album.id] ? 'Uploading…' : 'Upload'}
						</span>
						<input
							type="file"
							accept="image/jpeg,image/png,image/webp"
							class="sr-only"
							disabled={uploadingCover[group.album.id]}
							onchange={(e) => onCoverFileChange(group.album.id, e)}
						/>
					</label>
					<div class="min-w-0 flex-1">
						<h3 class="truncate text-base font-semibold">
							<a href="/admin/albums/{group.album.id}" class="hover:underline">
								{group.album.title}
							</a>
						</h3>
						<p class="truncate text-xs text-muted-foreground">
							{#if group.artist}
								<a href="/admin/artists/{group.artist.id}" class="hover:underline"
									>{group.artist.name}</a
								>
							{:else}
								(no artist)
							{/if}
							{#if group.album.date}· {group.album.date}{/if}
							· {group.tracks.length} track{group.tracks.length === 1 ? '' : 's'}
							{#if pendingTracks.length > 0}
								· <span class="text-amber-600 dark:text-amber-400"
									>{pendingTracks.length} pending</span
								>
							{/if}
						</p>
					</div>
					<Button
						size="sm"
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
						size="sm"
						variant="outline"
						onclick={() => lookupOnMb('album', group.album.id)}
						disabled={mbLooking[group.album.id]}
					>
						{mbLooking[group.album.id] ? '…' : 'Lookup on MB'}
					</Button>
					<Button
						size="sm"
						variant="ghost"
						onclick={() => (expanded[group.album.id] = !isExpanded)}
					>
						{isExpanded ? 'Collapse' : 'Expand'}
					</Button>
				</header>

				{#if albumSuggs.length}
					{@const mbOpen = mbExpanded[group.album.id] === true}
					<div class="border-b bg-muted/10">
						<button
							type="button"
							class="flex w-full items-center gap-2 px-4 py-2 text-xs font-semibold tracking-wide text-muted-foreground uppercase hover:text-foreground"
							onclick={() => (mbExpanded[group.album.id] = !mbOpen)}
						>
							{mbOpen ? '▾' : '▸'} MusicBrainz suggestions ({albumSuggs.length})
						</button>
						{#if mbOpen}
							<MbSuggestionList suggestions={albumSuggs} entityType="album" />
						{/if}
					</div>
				{/if}

				{#if isExpanded}
					<div>
						{#if !group.album.approved}
							<div class="border-b">
								<div class="flex items-center gap-3 px-4 py-2">
									<span
										class="rounded bg-amber-500/10 px-1.5 py-0.5 text-[10px] font-medium text-amber-600 dark:text-amber-400"
									>
										ALBUM
									</span>
									<a
										href="/admin/albums/{group.album.id}"
										class="min-w-0 flex-1 truncate text-sm hover:underline"
									>
										{group.album.title}
									</a>
									{#if group.album.sortTitle && group.album.sortTitle !== group.album.title}
										<span class="hidden truncate text-xs text-muted-foreground sm:inline"
											>sort: {group.album.sortTitle}</span
										>
									{/if}
									<Button
										size="sm"
										onclick={() => approveEntity('album', group.album.id)}
										disabled={approving[group.album.id]}
									>
										{approving[group.album.id] ? '…' : 'Approve'}
									</Button>
									<Button size="sm" variant="ghost" onclick={() => toggleEdit(group.album.id)}>
										{albumEditing ? 'Close' : 'Edit'}
									</Button>
								</div>
								{#if albumEditing}
									<div class="border-t bg-muted/30 p-4">
										<AlbumEditForm
											album={group.album}
											artist={group.artist}
											canDelete={group.tracks.length === 0}
											showArtistLink={false}
											onAfterChange={() => afterChange(group.album.id)}
											onAfterDelete={() => afterChange(group.album.id)}
										/>
									</div>
								{/if}
							</div>
						{/if}

						{#if group.artist && !group.artist.approved}
							{@const groupArtist = group.artist}
							{@const artistSuggs = suggestionsFor('artist', groupArtist.id)}
							{@const mbOpen = mbExpanded[groupArtist.id] === true}
							<div class="border-b">
								<div class="flex items-center gap-3 px-4 py-2">
									<span
										class="rounded bg-amber-500/10 px-1.5 py-0.5 text-[10px] font-medium text-amber-600 dark:text-amber-400"
									>
										ARTIST
									</span>
									<a
										href="/admin/artists/{groupArtist.id}"
										class="min-w-0 flex-1 truncate text-sm hover:underline"
									>
										{groupArtist.name}
									</a>
									{#if groupArtist.sortName && groupArtist.sortName !== groupArtist.name}
										<span class="hidden truncate text-xs text-muted-foreground sm:inline"
											>sort: {groupArtist.sortName}</span
										>
									{/if}
									<Button
										size="sm"
										onclick={() => approveEntity('artist', groupArtist.id)}
										disabled={approving[groupArtist.id]}
									>
										{approving[groupArtist.id] ? '…' : 'Approve'}
									</Button>
									<Button
										size="sm"
										variant="outline"
										onclick={() => lookupOnMb('artist', groupArtist.id)}
										disabled={mbLooking[groupArtist.id]}
									>
										{mbLooking[groupArtist.id] ? '…' : 'Lookup on MB'}
									</Button>
									<Button size="sm" variant="ghost" onclick={() => toggleEdit(groupArtist.id)}>
										{artistEditing ? 'Close' : 'Edit'}
									</Button>
								</div>
								{#if artistEditing}
									<div class="border-t bg-muted/30 p-4">
										<ArtistEditForm
											artist={groupArtist}
											canDelete={false}
											onAfterChange={() => afterChange(groupArtist.id)}
											onAfterDelete={() => afterChange(groupArtist.id)}
										/>
									</div>
								{/if}
								{#if artistSuggs.length}
									<div class="border-t bg-muted/10">
										<button
											type="button"
											class="flex w-full items-center gap-2 px-4 py-2 text-xs font-semibold tracking-wide text-muted-foreground uppercase hover:text-foreground"
											onclick={() => (mbExpanded[groupArtist.id] = !mbOpen)}
										>
											{mbOpen ? '▾' : '▸'} MusicBrainz suggestions ({artistSuggs.length})
										</button>
										{#if mbOpen}
											<MbSuggestionList suggestions={artistSuggs} entityType="artist" />
										{/if}
									</div>
								{/if}
							</div>
						{/if}

						{#each pendingTracks as track (track.id)}
							{@const trackEditing = editing[track.id] === true}
							{@const trackArtist = trackArtistFor(track.artistId)}
							<div class="border-b last:border-b-0">
								<div class="flex items-center gap-3 px-4 py-2">
									<span class="w-8 shrink-0 text-right font-mono text-xs text-muted-foreground">
										{track.trackNo ?? '–'}
									</span>
									<a
										href="/admin/tracks/{track.id}"
										class="min-w-0 flex-1 truncate text-sm hover:underline"
									>
										{track.title}
									</a>
									{#if trackArtist}
										<a
											href="/admin/artists/{trackArtist.id}"
											class="hidden shrink-0 truncate text-xs text-muted-foreground hover:underline sm:inline"
										>
											{trackArtist.name}
										</a>
									{/if}
									<span class="hidden shrink-0 text-xs text-muted-foreground sm:inline">
										{formatMilliseconds(Number(track.durationMs))}
									</span>
									<Button
										size="sm"
										onclick={() => approveEntity('track', track.id)}
										disabled={approving[track.id]}
									>
										{approving[track.id] ? '…' : 'Approve'}
									</Button>
									<Button size="sm" variant="ghost" onclick={() => toggleEdit(track.id)}>
										{trackEditing ? 'Close' : 'Edit'}
									</Button>
								</div>
								{#if trackEditing}
									<div class="border-t bg-muted/30 p-4">
										<TrackEditForm
											{track}
											artist={trackArtist}
											album={group.album}
											showArtistLink={false}
											showAlbumLink={false}
											onAfterChange={() => afterChange(track.id)}
											onAfterDelete={() => afterChange(track.id)}
										/>
									</div>
								{/if}
							</div>
						{/each}

						{#if approvedTracks.length > 0}
							<div class="border-t bg-muted/20 px-4 py-2">
								<button
									type="button"
									class="text-xs font-semibold tracking-wide text-muted-foreground uppercase hover:text-foreground"
									onclick={() => (showApproved[group.album.id] = !showApproved[group.album.id])}
								>
									{showApproved[group.album.id] ? '▾' : '▸'} Approved tracks ({approvedTracks.length})
								</button>
								{#if showApproved[group.album.id]}
									<ul class="mt-2 space-y-1 text-sm">
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

	{#if data.queue.standaloneArtists.length > 0}
		<section class="mx-auto max-w-4xl space-y-2">
			<h2 class="text-lg font-semibold">Pending artists ({data.queue.standaloneArtists.length})</h2>
			<div class="space-y-4">
				{#each data.queue.standaloneArtists as group (group.artist.id)}
					{@const artist = group.artist}
					{@const isEditing = editing[artist.id] === true}
					{@const isExpanded = expanded[artist.id] === true}
					{@const standaloneArtistSuggs = suggestionsFor('artist', artist.id)}
					{@const standaloneMbOpen = mbExpanded[artist.id] === true}
					{@const trackAlbumMap = new Map(group.trackAlbums.map((a) => [a.id, a]))}
					{@const artistTrackAlbumFor = (albumId: string | null) =>
						albumId == null ? null : (trackAlbumMap.get(albumId) ?? null)}
					<article class="overflow-hidden rounded-lg border bg-card shadow-sm">
						<div class="border-b last:border-b-0">
							<div class="flex items-center gap-3 px-4 py-2">
								<span
									class="rounded bg-amber-500/10 px-1.5 py-0.5 text-[10px] font-medium text-amber-600 dark:text-amber-400"
								>
									ARTIST
								</span>
								<a
									href="/admin/artists/{artist.id}"
									class="min-w-0 flex-1 truncate text-sm hover:underline"
								>
									{artist.name}
								</a>
								{#if artist.sortName && artist.sortName !== artist.name}
									<span class="hidden truncate text-xs text-muted-foreground sm:inline"
										>sort: {artist.sortName}</span
									>
								{/if}
								{#if group.tracks.length > 0}
									<span class="hidden shrink-0 text-xs text-muted-foreground sm:inline">
										{group.tracks.length} track{group.tracks.length === 1 ? '' : 's'}
									</span>
								{/if}
								<Button
									size="sm"
									onclick={() => approveEntity('artist', artist.id)}
									disabled={approving[artist.id]}
								>
									{approving[artist.id] ? '…' : 'Approve'}
								</Button>
								<Button
									size="sm"
									variant="outline"
									onclick={() => lookupOnMb('artist', artist.id)}
									disabled={mbLooking[artist.id]}
								>
									{mbLooking[artist.id] ? '…' : 'Lookup on MB'}
								</Button>
								<Button size="sm" variant="ghost" onclick={() => toggleEdit(artist.id)}>
									{isEditing ? 'Close' : 'Edit'}
								</Button>
								{#if group.tracks.length > 0}
									<Button
										size="sm"
										variant="ghost"
										onclick={() => (expanded[artist.id] = !isExpanded)}
									>
										{isExpanded ? 'Collapse' : 'Expand'}
									</Button>
								{/if}
							</div>
							{#if isEditing}
								<div class="border-t bg-muted/30 p-4">
									<ArtistEditForm
										{artist}
										canDelete={group.tracks.length === 0}
										onAfterChange={() => afterChange(artist.id)}
										onAfterDelete={() => afterChange(artist.id)}
									/>
								</div>
							{/if}
							{#if standaloneArtistSuggs.length}
								<div class="border-t bg-muted/10">
									<button
										type="button"
										class="flex w-full items-center gap-2 px-4 py-2 text-xs font-semibold tracking-wide text-muted-foreground uppercase hover:text-foreground"
										onclick={() => (mbExpanded[artist.id] = !standaloneMbOpen)}
									>
										{standaloneMbOpen ? '▾' : '▸'} MusicBrainz suggestions ({standaloneArtistSuggs.length})
									</button>
									{#if standaloneMbOpen}
										<MbSuggestionList suggestions={standaloneArtistSuggs} entityType="artist" />
									{/if}
								</div>
							{/if}
						</div>

						{#if isExpanded}
							{#each group.tracks as track (track.id)}
								{@const trackEditing = editing[track.id] === true}
								{@const trackAlbum = artistTrackAlbumFor(track.albumId)}
								<div class="border-b last:border-b-0">
									<div class="flex items-center gap-3 px-4 py-2">
										<span class="w-8 shrink-0 text-right font-mono text-xs text-muted-foreground">
											{track.trackNo ?? '–'}
										</span>
										<a
											href="/admin/tracks/{track.id}"
											class="min-w-0 flex-1 truncate text-sm hover:underline"
										>
											{track.title}
										</a>
										{#if trackAlbum}
											<a
												href="/admin/albums/{trackAlbum.id}"
												class="hidden shrink-0 truncate text-xs text-muted-foreground hover:underline sm:inline"
											>
												{trackAlbum.title}
											</a>
										{/if}
										<span class="hidden shrink-0 text-xs text-muted-foreground sm:inline">
											{formatMilliseconds(Number(track.durationMs))}
										</span>
										{#if !track.approved}
											<Button
												size="sm"
												onclick={() => approveEntity('track', track.id)}
												disabled={approving[track.id]}
											>
												{approving[track.id] ? '…' : 'Approve'}
											</Button>
										{/if}
										<Button size="sm" variant="ghost" onclick={() => toggleEdit(track.id)}>
											{trackEditing ? 'Close' : 'Edit'}
										</Button>
									</div>
									{#if trackEditing}
										<div class="border-t bg-muted/30 p-4">
											<TrackEditForm
												{track}
												{artist}
												album={trackAlbum}
												showArtistLink={false}
												onAfterChange={() => afterChange(track.id)}
												onAfterDelete={() => afterChange(track.id)}
											/>
										</div>
									{/if}
								</div>
							{/each}
						{/if}
					</article>
				{/each}
			</div>
		</section>
	{/if}
</div>
