import type { GetAlbumsResponse } from '$lib/bindings/response/album/get-albums-response';
import { extractPaginationFromUrl } from '$lib/utils/pagination';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch, url }) => {
	const { limit, offset, page } = extractPaginationFromUrl(url);
	const res = await fetch(`/api/albums?limit=${limit}&offset=${offset}`);

	if (!res.ok) {
		return { albums: null, error: { status: res.status, message: 'Server error' }, page };
	}

	return {
		albums: (await res.json()) as GetAlbumsResponse,
		error: null,
		page,
	};
};
