<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import {
		acceptSuggestion,
		asAlbumProposal,
		asArtistProposal,
		asTrackProposal,
		rejectSuggestion,
	} from '$lib/services/musicbrainz-service';
	import type { MetadataSuggestionResponse } from '$lib/bindings/response/admin/metadata-suggestion-response';
	import { toast } from 'svelte-sonner';
	import { invalidateAll } from '$app/navigation';

	type Props = {
		suggestions: MetadataSuggestionResponse[];
		/** 'album' | 'artist' | 'track' - which payload shape to render. */
		entityType: 'album' | 'artist' | 'track';
		coverArtBaseUrl?: string;
	};

	let {
		suggestions,
		entityType,
		coverArtBaseUrl = 'https://coverartarchive.org',
	}: Props = $props();

	let busy = $state<Record<string, boolean>>({});

	function thumbUrl(s: MetadataSuggestionResponse): string | null {
		if (entityType !== 'album') return null;
		const p = asAlbumProposal(s.proposed);
		if (!p.primaryReleaseMbid) return null;
		// CAA serves thumbs straight from this URL; the file we'll persist
		// only downloads on accept.
		return `${coverArtBaseUrl}/release/${p.primaryReleaseMbid}/front-250`;
	}

	async function onAccept(id: string) {
		busy[id] = true;
		const { error } = await acceptSuggestion(fetch, id);
		busy[id] = false;
		if (error) {
			toast.error('Failed to apply suggestion', { description: error });
		} else {
			toast.success('Applied MusicBrainz suggestion');
			await invalidateAll();
		}
	}

	async function onReject(id: string) {
		busy[id] = true;
		const { error } = await rejectSuggestion(fetch, id);
		busy[id] = false;
		if (error) {
			toast.error('Failed to reject', { description: error });
		} else {
			toast.success('Suggestion rejected');
			await invalidateAll();
		}
	}
</script>

{#if suggestions.length === 0}
	<p class="px-4 py-2 text-xs text-muted-foreground">No MusicBrainz suggestions yet.</p>
{:else}
	<ul class="divide-y">
		{#each suggestions as s (s.id)}
			{#if entityType === 'album'}
				{@const p = asAlbumProposal(s.proposed)}
				{@const thumb = thumbUrl(s)}
				<li class="flex items-start gap-3 px-4 py-3">
					{#if thumb}
						<img
							src={thumb}
							alt=""
							loading="lazy"
							referrerpolicy="no-referrer"
							class="size-16 shrink-0 rounded border bg-muted object-cover"
						/>
					{:else}
						<div class="size-16 shrink-0 rounded border bg-muted"></div>
					{/if}
					<div class="min-w-0 flex-1">
						<div class="flex flex-wrap items-baseline gap-2">
							<span class="truncate text-sm font-medium">{p.title ?? '(no title)'}</span>
							<span
								class="rounded bg-emerald-500/10 px-1.5 py-0.5 text-[10px] font-medium text-emerald-600 dark:text-emerald-400"
							>
								score {s.score}
							</span>
						</div>
						<p class="truncate text-xs text-muted-foreground">
							{#if p.artistName}{p.artistName}{:else}(no artist){/if}
							{#if p.date}· {p.date}{/if}
							{#if p.primaryReleaseCountry}· {p.primaryReleaseCountry}{/if}
						</p>
						<p class="truncate font-mono text-[10px] text-muted-foreground">
							mbid: {p.mbid ?? '—'}
						</p>
					</div>
					<div class="flex shrink-0 gap-1">
						<Button size="sm" onclick={() => onAccept(s.id)} disabled={busy[s.id]}>
							{busy[s.id] ? '…' : 'Accept'}
						</Button>
						<Button size="sm" variant="ghost" onclick={() => onReject(s.id)} disabled={busy[s.id]}>
							Reject
						</Button>
					</div>
				</li>
			{:else if entityType === 'artist'}
				{@const p = asArtistProposal(s.proposed)}
				<li class="flex items-start gap-3 px-4 py-3">
					<div class="min-w-0 flex-1">
						<div class="flex flex-wrap items-baseline gap-2">
							<span class="truncate text-sm font-medium">{p.name ?? '(no name)'}</span>
							<span
								class="rounded bg-emerald-500/10 px-1.5 py-0.5 text-[10px] font-medium text-emerald-600 dark:text-emerald-400"
							>
								score {s.score}
							</span>
						</div>
						<p class="truncate text-xs text-muted-foreground">
							{#if p.sortName}sort: {p.sortName}{/if}
							{#if p.country}· {p.country}{/if}
							{#if p.disambiguation}· {p.disambiguation}{/if}
						</p>
						<p class="truncate font-mono text-[10px] text-muted-foreground">
							mbid: {p.mbid ?? '—'}
						</p>
					</div>
					<div class="flex shrink-0 gap-1">
						<Button size="sm" onclick={() => onAccept(s.id)} disabled={busy[s.id]}>
							{busy[s.id] ? '…' : 'Accept'}
						</Button>
						<Button size="sm" variant="ghost" onclick={() => onReject(s.id)} disabled={busy[s.id]}>
							Reject
						</Button>
					</div>
				</li>
			{:else}
				{@const p = asTrackProposal(s.proposed)}
				<li class="flex items-start gap-3 px-4 py-3">
					<div class="min-w-0 flex-1">
						<div class="flex flex-wrap items-baseline gap-2">
							<span class="truncate text-sm font-medium">{p.title ?? '(no title)'}</span>
							<span
								class="rounded bg-emerald-500/10 px-1.5 py-0.5 text-[10px] font-medium text-emerald-600 dark:text-emerald-400"
							>
								score {s.score}
							</span>
						</div>
						<p class="truncate text-xs text-muted-foreground">
							{#if p.artistName}{p.artistName}{/if}
							{#if p.releaseTitle}· {p.releaseTitle}{/if}
						</p>
						<p class="truncate font-mono text-[10px] text-muted-foreground">
							mbid: {p.mbid ?? '—'}
						</p>
					</div>
					<div class="flex shrink-0 gap-1">
						<Button size="sm" onclick={() => onAccept(s.id)} disabled={busy[s.id]}>
							{busy[s.id] ? '…' : 'Accept'}
						</Button>
						<Button size="sm" variant="ghost" onclick={() => onReject(s.id)} disabled={busy[s.id]}>
							Reject
						</Button>
					</div>
				</li>
			{/if}
		{/each}
	</ul>
{/if}
