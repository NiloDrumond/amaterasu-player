import type { HandleFetch, HandleServerError } from '@sveltejs/kit';

// Force the real browser session cookie onto server-side `/api` fetches.
//
// In production the Bun server is pinned to ORIGIN=http://127.0.0.1:3000 (so
// `event.url` is plain http), while the session cookie is `Secure`. SvelteKit's
// implicit cookie forwarding strips `Secure` cookies from server-side fetches
// whose target is not https, so authenticated calls like /api/auth/me (in the
// protected layout load) arrived unauthenticated and bounced the user back to
// /login. We re-attach the raw incoming cookie to bypass that filter.
//
// We deliberately keep the request URL unchanged (same origin as `event.url`),
// otherwise SvelteKit stops propagating the backend's `Set-Cookie` back to the
// browser and login can no longer establish a session.
export const handleFetch: HandleFetch = async ({ event, request, fetch }) => {
	const url = new URL(request.url);

	if (url.pathname.startsWith('/api') || url.pathname.startsWith('/admin/logs')) {
		const headers = new Headers(request.headers);

		const cookie = event.request.headers.get('cookie');
		if (cookie) headers.set('cookie', cookie);

		// Let the backend see the original scheme (for its Secure-cookie flag).
		const xfProto = event.request.headers.get('x-forwarded-proto');
		if (xfProto) headers.set('x-forwarded-proto', xfProto);

		// Propagate the real client IP. Without this every server-side API call
		// reaches the backend as 127.0.0.1 (the Bun process), so the per-IP rate
		// limiter (SmartIpKeyExtractor) buckets *all* users together and trips
		// for everyone at once. nginx appends 127.0.0.1 to this on the /api hop,
		// and the extractor keeps the leftmost (real) address.
		const xfFor = event.request.headers.get('x-forwarded-for');
		if (xfFor) headers.set('x-forwarded-for', xfFor);

		return fetch(new Request(request, { headers }));
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
