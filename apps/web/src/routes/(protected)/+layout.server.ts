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
	const { data: user, error: errorMessage, status } = await getCurrentUser(fetch);
	// Only a genuine 401 means "not logged in" — redirect to the login page.
	// Any other failure (e.g. a transient 429 from the rate limiter, or a 5xx)
	// must NOT log the user out, or a momentary blip bounces them to /login.
	if (status === 401) {
		redirect(303, `/login?redirectTo=${url.pathname}`);
	}
	if (errorMessage || !user) {
		error(status && status >= 400 ? status : 500, errorMessage ?? 'Failed to load user');
	}
	const { data: recent } = await getRecentPlaylists(fetch, 10);
	return { user, recentPlaylists: recent ?? [] };
};
