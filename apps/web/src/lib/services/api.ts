export type Result<T> =
	| { data: T; error: null; status: number }
	| { data: null; error: string; status: number | null };

type Fetch = typeof fetch;

export async function api<T>(
	fetch: Fetch,
	url: string,
	options?: { method?: string; body?: unknown },
): Promise<Result<T>> {
	try {
		const fetchOptions: RequestInit = {};
		if (options?.method) fetchOptions.method = options.method;

		if (options?.body !== undefined) {
			fetchOptions.headers = { 'Content-Type': 'application/json' };
			fetchOptions.body = JSON.stringify(options.body);
		}

		const res = await fetch(url, fetchOptions);

		if (!res.ok) {
			let errorMessage: string;
			try {
				const body = await res.json();
				errorMessage = typeof body.error === 'string' ? body.error : res.statusText;
			} catch {
				errorMessage = res.statusText;
			}

			return {
				data: null,
				error: errorMessage,
				status: res.status,
			};
		}

		if (res.status === 204 || res.headers?.get('content-length') === '0') {
			return { data: null as unknown as T, error: null, status: res.status };
		}
		const data = (await res.json()) as T;
		return { data, error: null, status: res.status };
	} catch {
		return {
			data: null,
			error: 'Network error',
			status: null,
		};
	}
}
