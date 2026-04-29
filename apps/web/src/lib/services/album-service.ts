import type { GetAlbumsResponse } from '$lib/bindings/response/album/get-albums-response';
import type { AlbumResponse } from '$lib/bindings/response/album/album-response';
import type { TrackResponse } from '$lib/bindings/response/track/track-response';
import type { PaginationParams } from '$lib/bindings/request/common/pagination-params';
import { api, type Result } from './api';

type Fetch = typeof fetch;

export function getAlbums(
	fetch: Fetch,
	params?: PaginationParams,
): Promise<Result<GetAlbumsResponse>> {
	const query = params ? `?limit=${params.limit}&offset=${params.offset}` : '';
	return api<GetAlbumsResponse>(fetch, `/api/albums${query}`);
}

export function getAlbum(fetch: Fetch, id: string): Promise<Result<AlbumResponse>> {
	return api<AlbumResponse>(fetch, `/api/albums/${id}`);
}

export function getAlbumTracks(fetch: Fetch, id: string): Promise<Result<TrackResponse[]>> {
	return api<TrackResponse[]>(fetch, `/api/albums/${id}/tracks`);
}
