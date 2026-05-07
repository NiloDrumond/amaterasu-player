import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { getArtist, getArtistAlbums, getArtistTracks } from '$lib/services/artist-service';
import type { AlbumResponse } from '$lib/bindings/response/album/album-response';
import type { ArtistResponse } from '$lib/bindings/response/artist/artist-response';
import type { TrackResponse } from '$lib/bindings/response/track/track-response';

type LoadData = {
	artist: ArtistResponse;
	albums: AlbumResponse[];
	tracks: TrackResponse[];
};
export const load: PageServerLoad = async ({ fetch, params }): Promise<LoadData> => {
	const [artistResult, albumsResult, tracksResult] = await Promise.all([
		getArtist(fetch, params.id),
		getArtistAlbums(fetch, params.id),
		getArtistTracks(fetch, params.id),
	]);

	if (artistResult.status === 404) {
		error(404, 'Artist not found');
	}

	if (artistResult.error || albumsResult.error || tracksResult.error) {
		error(500, 'Failed to load artist');
	}

	if (!artistResult.data || !albumsResult.data || !tracksResult.data) {
		error(500, 'Failed to load artist');
	}

	return {
		artist: artistResult.data,
		albums: albumsResult.data,
		tracks: tracksResult.data,
	};
};
