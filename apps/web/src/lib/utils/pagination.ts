import type { SortDir } from '$lib/bindings/request/common/sort-dir';

export const DEFAULT_LIMIT = 50;

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
	seed: number | null;
} {
	const sort = url.searchParams.get('sort');
	const dir: SortDir = url.searchParams.get('dir') === 'desc' ? 'desc' : 'asc';
	const seedParam = url.searchParams.get('seed');
	let seed: number | null = null;
	if (seedParam) {
		const parsed = Number.parseInt(seedParam, 10);
		if (!Number.isNaN(parsed)) seed = parsed;
	}
	return { sort, dir, seed };
}

export function applySortToUrl(url: URL, sort: string | null, dir: SortDir): URL {
	if (sort) {
		url.searchParams.set('sort', sort);
		url.searchParams.set('dir', dir);
	} else {
		url.searchParams.delete('sort');
		url.searchParams.delete('dir');
	}
	url.searchParams.delete('seed');
	url.searchParams.delete('offset');
	return url;
}

export function applyRandomSortToUrl(url: URL, seed: number): URL {
	url.searchParams.set('sort', 'random');
	url.searchParams.delete('dir');
	url.searchParams.set('seed', seed.toString());
	url.searchParams.delete('offset');
	return url;
}

export function generateSeed(): number {
	return Math.floor(Math.random() * 2_147_483_647);
}
