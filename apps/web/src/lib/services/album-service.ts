import type { GetAlbumsResponse } from '$lib/bindings/response/album/get-albums-response';
import type { AlbumResponse } from '$lib/bindings/response/album/album-response';
import type { TrackResponse } from '$lib/bindings/response/track/track-response';
import type { AdminAlbumResponse } from '$lib/bindings/response/admin/admin-album-response';
import type { PaginationParams } from '$lib/bindings/request/common/pagination-params';
import type { SortDir } from '$lib/bindings/request/common/sort-dir';
import { api, type Result } from './api';

type Fetch = typeof fetch;

export interface GetAlbumsParams extends PaginationParams {
	f?: string | null;
	sort?: string | null;
	dir?: SortDir | null;
	seed?: number | null;
}

export function getAlbums(
	fetch: Fetch,
	params?: GetAlbumsParams,
): Promise<Result<GetAlbumsResponse>> {
	if (!params) return api<GetAlbumsResponse>(fetch, '/api/albums');
	const search = new URLSearchParams();
	search.set('limit', String(params.limit));
	search.set('offset', String(params.offset));
	if (params.f) search.set('f', params.f);
	if (params.sort) {
		search.set('sort', params.sort);
		search.set('dir', params.dir ?? 'asc');
	}
	if (params.seed != null) search.set('seed', String(params.seed));
	return api<GetAlbumsResponse>(fetch, `/api/albums?${search.toString()}`);
}

export function getAlbum(fetch: Fetch, id: string): Promise<Result<AlbumResponse>> {
	return api<AlbumResponse>(fetch, `/api/albums/${id}`);
}

export function getAlbumTracks(fetch: Fetch, id: string): Promise<Result<TrackResponse[]>> {
	return api<TrackResponse[]>(fetch, `/api/albums/${id}/tracks`);
}

export function searchAlbums(
	fetch: Fetch,
	q: string,
	limit = 10,
): Promise<Result<AdminAlbumResponse[]>> {
	const search = new URLSearchParams({ q, limit: String(limit) });
	return api<AdminAlbumResponse[]>(fetch, `/api/albums/search?${search.toString()}`);
}
