import type { SortDir } from '$lib/bindings/request/common/sort-dir';

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

export function extractSortFromUrl(url: URL): {
	sort: string | null;
	dir: SortDir;
} {
	const sort = url.searchParams.get('sort');
	const dir: SortDir = url.searchParams.get('dir') === 'desc' ? 'desc' : 'asc';
	return { sort, dir };
}

export function applySortToUrl(url: URL, sort: string | null, dir: SortDir): URL {
	if (sort) {
		url.searchParams.set('sort', sort);
		url.searchParams.set('dir', dir);
	} else {
		url.searchParams.delete('sort');
		url.searchParams.delete('dir');
	}
	url.searchParams.delete('offset');
	return url;
}
