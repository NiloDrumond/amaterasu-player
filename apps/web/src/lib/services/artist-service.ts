import type { GetArtistsResponse } from '$lib/bindings/response/artist/get-artists-response';
import type { ArtistResponse } from '$lib/bindings/response/artist/artist-response';
import type { AlbumResponse } from '$lib/bindings/response/album/album-response';
import type { TrackResponse } from '$lib/bindings/response/track/track-response';
import type { PaginationParams } from '$lib/bindings/request/common/pagination-params';
import { api, type Result } from './api';

type Fetch = typeof fetch;

export function getArtists(
	fetch: Fetch,
	params?: PaginationParams,
): Promise<Result<GetArtistsResponse>> {
	const query = params ? `?limit=${params.limit}&offset=${params.offset}` : '';
	return api<GetArtistsResponse>(fetch, `/api/artists${query}`);
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
