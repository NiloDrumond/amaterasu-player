import type { FilterNode } from '$lib/bindings/filter/filter-node';

/**
 * Encode a FilterNode tree to a URL-safe string for the `?f=` query parameter.
 * Empty groups (no children) collapse to `null` so the param is omitted entirely.
 */
export function encodeFilter(node: FilterNode | null): string | null {
	if (node === null) return null;
	if (node.kind === 'group' && node.children.length === 0) return null;
	const json = JSON.stringify(node, (_, v) => (typeof v === 'bigint' ? Number(v) : v));
	return base64UrlEncode(json);
}

export function decodeFilter(encoded: string | null | undefined): FilterNode | null {
	if (!encoded) return null;
	try {
		const json = base64UrlDecode(encoded);
		return JSON.parse(json) as FilterNode;
	} catch {
		return null;
	}
}

export function extractFilterFromUrl(url: URL): FilterNode | null {
	return decodeFilter(url.searchParams.get('f'));
}

function base64UrlEncode(input: string): string {
	const utf8 = new TextEncoder().encode(input);
	let binary = '';
	for (const byte of utf8) binary += String.fromCharCode(byte);
	return btoa(binary).replace(/\+/g, '-').replace(/\//g, '_').replace(/=+$/, '');
}

function base64UrlDecode(input: string): string {
	const padded = input.replace(/-/g, '+').replace(/_/g, '/');
	const padding = padded.length % 4 === 0 ? '' : '='.repeat(4 - (padded.length % 4));
	const binary = atob(padded + padding);
	const bytes = new Uint8Array(binary.length);
	for (let i = 0; i < binary.length; i++) bytes[i] = binary.charCodeAt(i);
	return new TextDecoder().decode(bytes);
}

/**
 * Convenience: returns the root group of an empty filter tree.
 */
export function emptyFilter(): FilterNode {
	return { kind: 'group', op: 'and', negate: false, children: [] };
}

/**
 * Reads the current text-search value out of a filter tree, if any.
 * Looks for a top-level non-negated `text contains` leaf (matches what
 * `setTextSearch` writes).
 */
export function getTextSearch(node: FilterNode | null): string {
	if (!node) return '';
	const children =
		node.kind === 'group' && node.op === 'and' && !node.negate ? node.children : [node];
	for (const c of children) {
		if (
			c.kind === 'leaf' &&
			c.field === 'text' &&
			c.op === 'contains' &&
			!c.negate &&
			c.value.kind === 'text'
		) {
			return c.value.value;
		}
	}
	return '';
}

/**
 * Returns a new filter tree with the top-level text search leaf set to `query`,
 * or removed if the query is empty. Other leaves are preserved.
 */
export function setTextSearch(node: FilterNode | null, query: string): FilterNode | null {
	const trimmed = query.trim();
	const root: FilterNode =
		node && node.kind === 'group' && node.op === 'and' && !node.negate
			? node
			: { kind: 'group', op: 'and', negate: false, children: node ? [node] : [] };

	const without = root.children.filter(
		(c) => !(c.kind === 'leaf' && c.field === 'text' && !c.negate && c.value.kind === 'text'),
	);

	const next: FilterNode[] = trimmed
		? [
				...without,
				{
					kind: 'leaf',
					field: 'text',
					op: 'contains',
					value: { kind: 'text', value: trimmed },
					negate: false,
				},
			]
		: without;

	if (next.length === 0) return null;
	return { kind: 'group', op: 'and', negate: false, children: next };
}
