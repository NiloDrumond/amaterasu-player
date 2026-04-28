import { error } from '@sveltejs/kit';
import type { AlbumResponse } from '$lib/bindings/response/album/album-response';
import type { TrackResponse } from '$lib/bindings/response/track/track-response';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch, params }) => {
	const [albumRes, tracksRes] = await Promise.all([
		fetch(`/api/albums/${params.id}`),
		fetch(`/api/albums/${params.id}/tracks`),
	]);

	if (albumRes.status === 404) {
		error(404, 'Album not found');
	}

	if (!albumRes.ok || !tracksRes.ok) {
		error(500, 'Failed to load album');
	}

	return {
		album: (await albumRes.json()) as AlbumResponse,
		tracks: (await tracksRes.json()) as TrackResponse[],
	};
};
