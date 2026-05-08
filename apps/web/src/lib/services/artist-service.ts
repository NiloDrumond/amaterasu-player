import type { GetArtistsResponse } from '$lib/bindings/response/artist/get-artists-response';
import type { ArtistResponse } from '$lib/bindings/response/artist/artist-response';
import type { AlbumResponse } from '$lib/bindings/response/album/album-response';
import type { TrackResponse } from '$lib/bindings/response/track/track-response';
import type { AdminArtistResponse } from '$lib/bindings/response/admin/admin-artist-response';
import type { PaginationParams } from '$lib/bindings/request/common/pagination-params';
import { api, type Result } from './api';

type Fetch = typeof fetch;

export interface GetArtistsParams extends PaginationParams {
	q?: string | null;
}

export function getArtists(
	fetch: Fetch,
	params?: GetArtistsParams,
): Promise<Result<GetArtistsResponse>> {
	if (!params) return api<GetArtistsResponse>(fetch, '/api/artists');
	const search = new URLSearchParams();
	search.set('limit', String(params.limit));
	search.set('offset', String(params.offset));
	if (params.q) search.set('q', params.q);
	return api<GetArtistsResponse>(fetch, `/api/artists?${search.toString()}`);
}

export function getArtist(fetch: Fetch, id: string): Promise<Result<ArtistResponse>> {
	return api<ArtistResponse>(fetch, `/api/artists/${id}`);
}

export function getArtistAlbums(fetch: Fetch, id: string): Promise<Result<AlbumResponse[]>> {
	return api<AlbumResponse[]>(fetch, `/api/artists/${id}/albums`);
}

export function getArtistTracks(fetch: Fetch, id: string): Promise<Result<TrackResponse[]>> {
	return api<TrackResponse[]>(fetch, `/api/artists/${id}/tracks`);
}

export function searchArtists(
	fetch: Fetch,
	q: string,
	limit = 10,
): Promise<Result<AdminArtistResponse[]>> {
	const search = new URLSearchParams({ q, limit: String(limit) });
	return api<AdminArtistResponse[]>(fetch, `/api/artists/search?${search.toString()}`);
}
