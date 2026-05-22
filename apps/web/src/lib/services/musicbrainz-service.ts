import type { AdminAlbumResponse } from '$lib/bindings/response/admin/admin-album-response';
import type { AdminArtistResponse } from '$lib/bindings/response/admin/admin-artist-response';
import type { AdminTrackResponse } from '$lib/bindings/response/admin/admin-track-response';
import type { BulkMbLookupResponse } from '$lib/bindings/response/admin/bulk-mb-lookup-response';
import type { MetadataSuggestionResponse } from '$lib/bindings/response/admin/metadata-suggestion-response';
import { api, type Result } from './api';

type Fetch = typeof fetch;

// ------------------------------------------------------------------
// Typed payloads. The server stores `proposed` as JSONB and the binding
// types it as `Record<string, unknown>`; these shapes describe what we
// actually put in there per `entityType`.
// ------------------------------------------------------------------

export type AlbumProposal = {
	mbid: string;
	releaseGroupMbid: string;
	title?: string | null;
	sortTitle?: string | null;
	date?: string | null;
	artistMbid?: string | null;
	artistName?: string | null;
	primaryReleaseMbid: string;
	primaryReleaseCountry?: string | null;
};

export type ArtistProposal = {
	mbid: string;
	name?: string | null;
	sortName?: string | null;
	country?: string | null;
	disambiguation?: string | null;
};

export type TrackProposal = {
	mbid: string;
	title?: string | null;
	artistMbid?: string | null;
	artistName?: string | null;
	releaseMbid?: string | null;
	releaseTitle?: string | null;
	lengthMs?: number | null;
};

export function asAlbumProposal(p: Record<string, unknown>): AlbumProposal {
	return p as unknown as AlbumProposal;
}

export function asArtistProposal(p: Record<string, unknown>): ArtistProposal {
	return p as unknown as ArtistProposal;
}

export function asTrackProposal(p: Record<string, unknown>): TrackProposal {
	return p as unknown as TrackProposal;
}

// ------------------------------------------------------------------
// Lookup triggers (enqueue background work).
// ------------------------------------------------------------------

export function lookupAlbum(fetch: Fetch, id: string): Promise<Result<void>> {
	return api<void>(fetch, `/api/admin/albums/${id}/mb-lookup`, { method: 'POST' });
}

export function lookupArtist(fetch: Fetch, id: string): Promise<Result<void>> {
	return api<void>(fetch, `/api/admin/artists/${id}/mb-lookup`, { method: 'POST' });
}

export function lookupTrack(fetch: Fetch, id: string): Promise<Result<void>> {
	return api<void>(fetch, `/api/admin/tracks/${id}/mb-lookup`, { method: 'POST' });
}

export function bulkLookupPending(fetch: Fetch): Promise<Result<BulkMbLookupResponse>> {
	return api<BulkMbLookupResponse>(fetch, '/api/admin/mb-lookup/pending', { method: 'POST' });
}

// ------------------------------------------------------------------
// Read suggestions.
// ------------------------------------------------------------------

export function getAlbumSuggestions(
	fetch: Fetch,
	id: string,
): Promise<Result<MetadataSuggestionResponse[]>> {
	return api<MetadataSuggestionResponse[]>(fetch, `/api/admin/albums/${id}/mb-suggestions`);
}

export function getArtistSuggestions(
	fetch: Fetch,
	id: string,
): Promise<Result<MetadataSuggestionResponse[]>> {
	return api<MetadataSuggestionResponse[]>(fetch, `/api/admin/artists/${id}/mb-suggestions`);
}

export function getTrackSuggestions(
	fetch: Fetch,
	id: string,
): Promise<Result<MetadataSuggestionResponse[]>> {
	return api<MetadataSuggestionResponse[]>(fetch, `/api/admin/tracks/${id}/mb-suggestions`);
}

/**
 * Batched fetch for the review queue page-server load. Returns pending album
 * suggestions for all given album IDs in a single round-trip.
 */
export function getReviewQueueAlbumSuggestions(
	fetch: Fetch,
	albumIds: string[],
): Promise<Result<MetadataSuggestionResponse[]>> {
	if (albumIds.length === 0) {
		return Promise.resolve({
			data: [] as MetadataSuggestionResponse[],
			error: null,
			status: 200,
		});
	}
	const qs = new URLSearchParams({ albumIds: albumIds.join(',') });
	return api<MetadataSuggestionResponse[]>(fetch, `/api/admin/review/queue/mb-suggestions?${qs}`);
}

// ------------------------------------------------------------------
// Accept / reject.
// ------------------------------------------------------------------

/**
 * Accepts the suggestion. The response shape depends on the entity type the
 * suggestion targets -- album/artist/track admin response. Use the
 * suggestion's `entityType` to discriminate on the call site.
 */
export function acceptSuggestion(
	fetch: Fetch,
	id: string,
): Promise<Result<AdminAlbumResponse | AdminArtistResponse | AdminTrackResponse>> {
	return api<AdminAlbumResponse | AdminArtistResponse | AdminTrackResponse>(
		fetch,
		`/api/admin/mb-suggestions/${id}/accept`,
		{ method: 'POST' },
	);
}

export function rejectSuggestion(fetch: Fetch, id: string): Promise<Result<void>> {
	return api<void>(fetch, `/api/admin/mb-suggestions/${id}/reject`, { method: 'POST' });
}
