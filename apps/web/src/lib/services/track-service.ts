import type { GetTracksResponse } from '$lib/bindings/response/track/get-tracks-response';
import type { PaginationParams } from '$lib/bindings/request/common/pagination-params';
import type { SortDir } from '$lib/bindings/request/common/sort-dir';
import { api, type Result } from './api';

type Fetch = typeof fetch;

export interface GetTracksParams extends PaginationParams {
	/** base64url-encoded JSON FilterNode */
	f?: string | null;
	sort?: string | null;
	dir?: SortDir | null;
}

export function getTracks(
	fetch: Fetch,
	params?: GetTracksParams,
): Promise<Result<GetTracksResponse>> {
	if (!params) return api<GetTracksResponse>(fetch, '/api/tracks');
	const search = new URLSearchParams();
	search.set('limit', String(params.limit));
	search.set('offset', String(params.offset));
	if (params.f) search.set('f', params.f);
	if (params.sort) {
		search.set('sort', params.sort);
		search.set('dir', params.dir ?? 'asc');
	}
	return api<GetTracksResponse>(fetch, `/api/tracks?${search.toString()}`);
}
