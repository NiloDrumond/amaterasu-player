import type { GetTracksResponse } from '$lib/bindings/response/track/get-tracks-response';
import { extractPaginationFromUrl } from '$lib/utils/pagination';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch, url }) => {
	const { limit, offset, page } = extractPaginationFromUrl(url);
	const res = await fetch(`/api/tracks?limit=${limit}&offset=${offset}`);

	if (!res.ok) {
		return { tracks: null, error: { status: res.status, message: 'Server error' }, page };
	}

	return {
		tracks: (await res.json()) as GetTracksResponse,
		error: null,
		page,
	};
};
