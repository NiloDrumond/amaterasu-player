import type { LayoutServerLoad } from './$types';
import { error, redirect } from '@sveltejs/kit';
import { getCurrentUser } from '$lib/services/auth-service';
import { getRecentPlaylists } from '$lib/services/playlist-service';
import type { CurrentUserResponse } from '$lib/bindings/response/auth/current-user-response';
import type { RecentPlaylistResponse } from '$lib/bindings/response/playlist/recent-playlist-response';

type LoadData = {
	user: CurrentUserResponse;
	recentPlaylists: RecentPlaylistResponse[];
};
export const load: LayoutServerLoad = async ({ fetch, url }): Promise<LoadData> => {
	const { data: user, error: errorMessage } = await getCurrentUser(fetch);
	if (errorMessage) {
		redirect(303, `/login?redirectTo=${url.pathname}`);
	}
	if (!user) {
		error(500, 'Failed to load user');
	}
	const { data: recent } = await getRecentPlaylists(fetch, 10);
	return { user, recentPlaylists: recent ?? [] };
};
