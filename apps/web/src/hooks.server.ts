import type { HandleFetch, HandleServerError } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

// Direct address of the Rust API inside the container / dev box.
const API_ORIGIN = env.API_ORIGIN ?? 'http://127.0.0.1:8080';

// Route server-side `/api` (and `/admin/logs`) fetches straight to the backend
// and forward the session cookie + proxy headers explicitly. SvelteKit's
// implicit same-origin cookie forwarding breaks in production because the Bun
// server is pinned to ORIGIN=http://127.0.0.1:3000 while the real request comes
// in as https://music.akaiamaterasu.com — so authenticated server-side calls
// (e.g. /api/auth/me in the protected layout load) arrived unauthenticated and
// bounced the user back to /login. Forwarding explicitly removes that
// dependency on ORIGIN / scheme / same-origin heuristics.
export const handleFetch: HandleFetch = async ({ event, request, fetch }) => {
	const url = new URL(request.url);

	if (url.pathname.startsWith('/api') || url.pathname.startsWith('/admin/logs')) {
		const target = API_ORIGIN + url.pathname + url.search;
		const proxied = new Request(target, request);

		const cookie = event.request.headers.get('cookie');
		if (cookie) proxied.headers.set('cookie', cookie);

		const xfProto = event.request.headers.get('x-forwarded-proto');
		if (xfProto) proxied.headers.set('x-forwarded-proto', xfProto);

		const xfFor = event.request.headers.get('x-forwarded-for');
		if (xfFor) proxied.headers.set('x-forwarded-for', xfFor);

		return fetch(proxied);
	}

	return fetch(request);
};

// SSR errors run in the SvelteKit (Bun) server, whose stdout is not shipped to
// Loki. Forward them to the backend ingest endpoint so they land in the same
// `service=amaterasu-web` stream as browser errors. `event.fetch` resolves
// `/api/*` against the incoming origin (through nginx → backend) and carries
// the session cookie, exactly like the load-function API calls do.
export const handleError: HandleServerError = async ({ error, event, status, message }) => {
	if (status !== 404) {
		try {
			await event.fetch('/api/client-logs', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					entries: [
						{
							level: 'error',
							message: error instanceof Error ? error.message : String(message),
							stack: error instanceof Error ? error.stack : undefined,
							url: event.url.href,
							route: event.route?.id ?? undefined,
							ts: Date.now(),
							context: { source: 'ssr', status },
						},
					],
				}),
			});
		} catch {
			// Never let error reporting mask or replace the original error.
		}
	}
	return { message };
};
