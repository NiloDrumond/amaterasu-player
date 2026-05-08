import type { GetTracksResponse } from '$lib/bindings/response/track/get-tracks-response';
import type { PaginationParams } from '$lib/bindings/request/common/pagination-params';
import { api, type Result } from './api';

type Fetch = typeof fetch;

export interface GetTracksParams extends PaginationParams {
	/** base64url-encoded JSON FilterNode */
	f?: string | null;
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
	return api<GetTracksResponse>(fetch, `/api/tracks?${search.toString()}`);
}
