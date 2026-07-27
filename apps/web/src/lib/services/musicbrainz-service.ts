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

// Every field is nullable: the payload is whatever MusicBrainz returned when
// the suggestion was enqueued, and nothing validates it on the way in. The
// readers below narrow at runtime so a missing or wrong-typed field surfaces
// as `null` here rather than as `undefined` at a call site.
export type AlbumProposal = {
	mbid: string | null;
	releaseGroupMbid: string | null;
	title: string | null;
	sortTitle: string | null;
	date: string | null;
	artistMbid: string | null;
	artistName: string | null;
	primaryReleaseMbid: string | null;
	primaryReleaseCountry: string | null;
};

export type ArtistProposal = {
	mbid: string | null;
	name: string | null;
	sortName: string | null;
	country: string | null;
	disambiguation: string | null;
};

export type TrackProposal = {
	mbid: string | null;
	title: string | null;
	artistMbid: string | null;
	artistName: string | null;
	releaseMbid: string | null;
	releaseTitle: string | null;
	lengthMs: number | null;
};

function str(v: unknown): string | null {
	return typeof v === 'string' && v.length > 0 ? v : null;
}

function num(v: unknown): number | null {
	return typeof v === 'number' && Number.isFinite(v) ? v : null;
}

export function asAlbumProposal(p: Record<string, unknown>): AlbumProposal {
	return {
		mbid: str(p.mbid),
		releaseGroupMbid: str(p.releaseGroupMbid),
		title: str(p.title),
		sortTitle: str(p.sortTitle),
		date: str(p.date),
		artistMbid: str(p.artistMbid),
		artistName: str(p.artistName),
		primaryReleaseMbid: str(p.primaryReleaseMbid),
		primaryReleaseCountry: str(p.primaryReleaseCountry),
	};
}

export function asArtistProposal(p: Record<string, unknown>): ArtistProposal {
	return {
		mbid: str(p.mbid),
		name: str(p.name),
		sortName: str(p.sortName),
		country: str(p.country),
		disambiguation: str(p.disambiguation),
	};
}

export function asTrackProposal(p: Record<string, unknown>): TrackProposal {
	return {
		mbid: str(p.mbid),
		title: str(p.title),
		artistMbid: str(p.artistMbid),
		artistName: str(p.artistName),
		releaseMbid: str(p.releaseMbid),
		releaseTitle: str(p.releaseTitle),
		lengthMs: num(p.lengthMs),
	};
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
 * AND artist suggestions in a single round-trip; each row carries
 * `entityType` so the client can bucket.
 */
export function getReviewQueueMbSuggestions(
	fetch: Fetch,
	ids: { albumIds: string[]; artistIds: string[] },
): Promise<Result<MetadataSuggestionResponse[]>> {
	if (ids.albumIds.length === 0 && ids.artistIds.length === 0) {
		return Promise.resolve({
			data: [] as MetadataSuggestionResponse[],
			error: null,
			status: 200,
		});
	}
	const qs = new URLSearchParams();
	if (ids.albumIds.length > 0) qs.set('albumIds', ids.albumIds.join(','));
	if (ids.artistIds.length > 0) qs.set('artistIds', ids.artistIds.join(','));
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
