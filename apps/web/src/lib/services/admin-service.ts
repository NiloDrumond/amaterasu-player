import type { AdminTrackResponse } from '$lib/bindings/response/admin/admin-track-response';
import type { AdminAlbumResponse } from '$lib/bindings/response/admin/admin-album-response';
import type { AdminArtistResponse } from '$lib/bindings/response/admin/admin-artist-response';
import type { AdminDeletedTrackResponse } from '$lib/bindings/response/admin/admin-deleted-track-response';
import { api, type Result } from './api';

type Fetch = typeof fetch;

export function scanLibrary(fetch: Fetch): Promise<Result<void>> {
	return api<void>(fetch, '/api/admin/scan-library', { method: 'POST' });
}

// ============================================================
// Track admin
// ============================================================

export function getAdminTrack(fetch: Fetch, id: string): Promise<Result<AdminTrackResponse>> {
	return api<AdminTrackResponse>(fetch, `/api/admin/tracks/${id}`);
}

/**
 * Tri-state field for partial track updates:
 *  - omit the key entirely → leave unchanged
 *  - `null` → clear the column
 *  - any value → set
 */
export type UpdateTrackBody = {
	title?: string;
	sortTitle?: string;
	artistId?: string | null;
	albumId?: string | null;
	disc?: number | null;
	trackNo?: number | null;
	date?: string | null;
	composer?: string | null;
	comment?: string | null;
};

export function updateAdminTrack(
	fetch: Fetch,
	id: string,
	body: UpdateTrackBody,
): Promise<Result<AdminTrackResponse>> {
	return api<AdminTrackResponse>(fetch, `/api/admin/tracks/${id}`, {
		method: 'PATCH',
		body,
	});
}

export type BatchUpdateTracksResult = { updated: number };

export function batchUpdateAdminTracks(
	fetch: Fetch,
	ids: string[],
	patch: UpdateTrackBody,
): Promise<Result<BatchUpdateTracksResult>> {
	return api<BatchUpdateTracksResult>(fetch, `/api/admin/tracks/batch`, {
		method: 'PATCH',
		body: { ids, patch },
	});
}

export function softDeleteAdminTrack(fetch: Fetch, id: string): Promise<Result<void>> {
	return api<void>(fetch, `/api/admin/tracks/${id}`, { method: 'DELETE' });
}

export function hardDeleteAdminTrack(fetch: Fetch, id: string): Promise<Result<void>> {
	return api<void>(fetch, `/api/admin/tracks/${id}?hard=true`, { method: 'DELETE' });
}

export function restoreAdminTrack(fetch: Fetch, id: string): Promise<Result<void>> {
	return api<void>(fetch, `/api/admin/tracks/${id}/restore`, { method: 'POST' });
}

export function forceRescanTrack(fetch: Fetch, id: string): Promise<Result<void>> {
	return api<void>(fetch, `/api/admin/tracks/${id}/force-rescan`, { method: 'POST' });
}

export function listDeletedTracks(fetch: Fetch): Promise<Result<AdminDeletedTrackResponse[]>> {
	return api<AdminDeletedTrackResponse[]>(fetch, '/api/admin/tracks/deleted');
}

// ============================================================
// Album admin
// ============================================================

export function getAdminAlbum(fetch: Fetch, id: string): Promise<Result<AdminAlbumResponse>> {
	return api<AdminAlbumResponse>(fetch, `/api/admin/albums/${id}`);
}

export type UpdateAlbumBody = {
	title: string;
	sortTitle: string;
	artistId: string | null;
	date: string | null;
};

export function updateAdminAlbum(
	fetch: Fetch,
	id: string,
	body: UpdateAlbumBody,
): Promise<Result<AdminAlbumResponse>> {
	return api<AdminAlbumResponse>(fetch, `/api/admin/albums/${id}`, {
		method: 'PATCH',
		body,
	});
}

export type CreateAlbumBody = {
	title: string;
	sortTitle?: string;
	artistId?: string | null;
	date?: string | null;
};

export function createAdminAlbum(
	fetch: Fetch,
	body: CreateAlbumBody,
): Promise<Result<AdminAlbumResponse>> {
	return api<AdminAlbumResponse>(fetch, '/api/admin/albums', { method: 'POST', body });
}

export function deleteAdminAlbum(fetch: Fetch, id: string): Promise<Result<void>> {
	return api<void>(fetch, `/api/admin/albums/${id}`, { method: 'DELETE' });
}

export function forceRescanAlbum(fetch: Fetch, id: string): Promise<Result<void>> {
	return api<void>(fetch, `/api/admin/albums/${id}/force-rescan`, { method: 'POST' });
}

// ============================================================
// Artist admin
// ============================================================

export function getAdminArtist(fetch: Fetch, id: string): Promise<Result<AdminArtistResponse>> {
	return api<AdminArtistResponse>(fetch, `/api/admin/artists/${id}`);
}

export type UpdateArtistBody = {
	name: string;
	sortName: string;
};

export function updateAdminArtist(
	fetch: Fetch,
	id: string,
	body: UpdateArtistBody,
): Promise<Result<AdminArtistResponse>> {
	return api<AdminArtistResponse>(fetch, `/api/admin/artists/${id}`, {
		method: 'PATCH',
		body,
	});
}

export type CreateArtistBody = {
	name: string;
	sortName?: string;
};

export function createAdminArtist(
	fetch: Fetch,
	body: CreateArtistBody,
): Promise<Result<AdminArtistResponse>> {
	return api<AdminArtistResponse>(fetch, '/api/admin/artists', { method: 'POST', body });
}

export function deleteAdminArtist(fetch: Fetch, id: string): Promise<Result<void>> {
	return api<void>(fetch, `/api/admin/artists/${id}`, { method: 'DELETE' });
}

export function forceRescanArtist(fetch: Fetch, id: string): Promise<Result<void>> {
	return api<void>(fetch, `/api/admin/artists/${id}/force-rescan`, { method: 'POST' });
}

export type MergeArtistBody = {
	sourceId: string;
	name: string;
	sortName: string;
};

export function mergeAdminArtist(
	fetch: Fetch,
	targetId: string,
	body: MergeArtistBody,
): Promise<Result<AdminArtistResponse>> {
	return api<AdminArtistResponse>(fetch, `/api/admin/artists/${targetId}/merge`, {
		method: 'POST',
		body,
	});
}

export type MergeAlbumBody = {
	sourceId: string;
	title: string;
	sortTitle: string;
	artistId: string | null;
	date: string | null;
	coverPath?: string | null;
};

export function mergeAdminAlbum(
	fetch: Fetch,
	targetId: string,
	body: MergeAlbumBody,
): Promise<Result<AdminAlbumResponse>> {
	return api<AdminAlbumResponse>(fetch, `/api/admin/albums/${targetId}/merge`, {
		method: 'POST',
		body,
	});
}

// ============================================================
// Picker search
// ============================================================

export function searchArtists(
	fetch: Fetch,
	q: string,
	limit = 20,
): Promise<Result<AdminArtistResponse[]>> {
	const params = new URLSearchParams({ q, limit: String(limit) });
	return api<AdminArtistResponse[]>(fetch, `/api/artists/search?${params}`);
}

export function searchAlbums(
	fetch: Fetch,
	q: string,
	opts: { limit?: number; artistId?: string | null } = {},
): Promise<Result<AdminAlbumResponse[]>> {
	const params = new URLSearchParams({ q, limit: String(opts.limit ?? 20) });
	if (opts.artistId) params.set('artistId', opts.artistId);
	return api<AdminAlbumResponse[]>(fetch, `/api/albums/search?${params}`);
}
