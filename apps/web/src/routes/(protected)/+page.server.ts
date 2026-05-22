import type { PageServerLoad } from './$types';
import {
	getForgottenFavorites,
	getListenAgain,
	getPinnedPlaylists,
	getRecentlyAddedAlbums,
} from '$lib/services/home-service';
import type { AlbumResponse } from '$lib/bindings/response/album/album-response';
import type { HomeItemResponse } from '$lib/bindings/response/me/home-item-response';
import type { PlaylistResponse } from '$lib/bindings/response/playlist/playlist-response';

type LoadData = {
	pinnedPlaylists: PlaylistResponse[];
	listenAgain: HomeItemResponse[];
	forgottenFavorites: HomeItemResponse[];
	recentlyAdded: AlbumResponse[];
};

export const load: PageServerLoad = async ({ fetch }): Promise<LoadData> => {
	const [pinned, listenAgain, forgotten, recentlyAdded] = await Promise.all([
		getPinnedPlaylists(fetch),
		getListenAgain(fetch, 20),
		getForgottenFavorites(fetch, 20),
		getRecentlyAddedAlbums(fetch, 20),
	]);
	return {
		pinnedPlaylists: pinned.data ?? [],
		listenAgain: listenAgain.data ?? [],
		forgottenFavorites: forgotten.data ?? [],
		recentlyAdded: recentlyAdded.data ?? [],
	};
};
