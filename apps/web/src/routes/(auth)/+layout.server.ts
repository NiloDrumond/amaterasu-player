import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ fetch }) => {
	const res = await fetch('/api/auth/me');
	if (res.ok) {
		redirect(303, '/');
	}
};
