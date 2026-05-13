import type { PaletteSearchResponse } from '$lib/bindings/response/search/palette-search-response';
import type { SearchEntityType } from '$lib/bindings/response/search/search-entity-type';
import type { SearchHit } from '$lib/bindings/response/search/search-hit';
import { api, type Result } from './api';

type Fetch = typeof fetch;

export type { PaletteSearchResponse, SearchEntityType, SearchHit };

export function searchPalette(
	fetch: Fetch,
	q: string,
	limit = 5,
): Promise<Result<PaletteSearchResponse>> {
	const params = new URLSearchParams({ q, limit: String(limit) });
	return api<PaletteSearchResponse>(fetch, `/api/search?${params.toString()}`);
}

export interface SearchAllParams {
	q: string;
	type: SearchEntityType;
	page?: number;
	pageSize?: number;
}

export function searchAll(fetch: Fetch, params: SearchAllParams): Promise<Result<SearchHit[]>> {
	const qs = new URLSearchParams({
		q: params.q,
		type: params.type,
		page: String(params.page ?? 1),
		page_size: String(params.pageSize ?? 25),
	});
	return api<SearchHit[]>(fetch, `/api/search/all?${qs.toString()}`);
}
