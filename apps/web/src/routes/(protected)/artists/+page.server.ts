import type { GetArtistsResponse } from '$lib/bindings/response/artist/get-artists-response';
import { extractPaginationFromUrl } from '$lib/utils/pagination';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch, url }) => {
	const { limit, offset, page } = extractPaginationFromUrl(url);
	const res = await fetch(`/api/artists?limit=${limit}&offset=${offset}`);

	if (!res.ok) {
		return { artists: null, error: { status: res.status, message: 'Server error' }, page };
	}

	return {
		artists: (await res.json()) as GetArtistsResponse,
		error: null,
		page,
	};
};
