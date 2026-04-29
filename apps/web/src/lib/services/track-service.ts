import type { GetTracksResponse } from '$lib/bindings/response/track/get-tracks-response';
import type { PaginationParams } from '$lib/bindings/request/common/pagination-params';
import { api, type Result } from './api';

type Fetch = typeof fetch;

export function getTracks(
	fetch: Fetch,
	params?: PaginationParams,
): Promise<Result<GetTracksResponse>> {
	const query = params ? `?limit=${params.limit}&offset=${params.offset}` : '';
	return api<GetTracksResponse>(fetch, `/api/tracks${query}`);
}
