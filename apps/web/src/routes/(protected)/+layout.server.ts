import type { LayoutServerLoad } from './$types';
import { error, redirect } from '@sveltejs/kit';
import { getCurrentUser } from '$lib/services/auth-service';

export const load: LayoutServerLoad = async ({ fetch, url }) => {
	const { data: user, error: errorMessage } = await getCurrentUser(fetch);
	if (errorMessage) {
		redirect(303, `/login?redirectTo=${url.pathname}`);
	}
	if (user) {
		error(500, 'Failed to load user');
	}
	return { user: user };
};
