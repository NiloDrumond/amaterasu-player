export const DEFAULT_LIMIT = 32;

export function extractPaginationFromUrl(url: URL): {
	limit: number;
	offset: number;
	page: number;
} {
	let limit: number = DEFAULT_LIMIT;
	const limitParam = url.searchParams.get('limit');
	if (limitParam) {
		const parsedLimit = Number.parseInt(limitParam);
		if (!Number.isNaN(parsedLimit)) {
			limit = parsedLimit;
		}
	}

	let offset: number = 0;
	const offsetParam = url.searchParams.get('offset');
	if (offsetParam) {
		const parsedOffset = Number.parseInt(offsetParam);
		if (!Number.isNaN(parsedOffset)) {
			offset = parsedOffset;
		}
	}

	const page = offset / limit + 1;
	return { limit, offset, page };
}
