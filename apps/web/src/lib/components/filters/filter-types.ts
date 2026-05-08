import type { Field } from '$lib/bindings/filter/field';
import type { FilterNode } from '$lib/bindings/filter/filter-node';
import type { FilterValue } from '$lib/bindings/filter/filter-value';
import type { Operator } from '$lib/bindings/filter/operator';

export type Entity = 'tracks' | 'albums';

export interface FieldSpec {
	field: Field;
	label: string;
	/** Default operator for the field. */
	defaultOp: Operator;
	/** Kind of value editor to show. */
	editor: 'text' | 'tag' | 'album' | 'artist' | 'intRange' | 'yearRange';
}

// `text` is not listed here — text search lives in the always-visible
// SearchInput, not as a chip in the FilterBar popover.
export const TRACK_FIELDS: FieldSpec[] = [
	{ field: 'tag', label: 'Tag', defaultOp: 'eq', editor: 'tag' },
	{ field: 'album', label: 'Album', defaultOp: 'eq', editor: 'album' },
	{ field: 'artist', label: 'Artist', defaultOp: 'eq', editor: 'artist' },
	{ field: 'year', label: 'Year', defaultOp: 'between', editor: 'yearRange' },
	{ field: 'durationMs', label: 'Duration', defaultOp: 'between', editor: 'intRange' },
];

export const ALBUM_FIELDS: FieldSpec[] = [
	{ field: 'tag', label: 'Tag', defaultOp: 'eq', editor: 'tag' },
	{ field: 'artist', label: 'Artist', defaultOp: 'eq', editor: 'artist' },
	{ field: 'year', label: 'Year', defaultOp: 'between', editor: 'yearRange' },
];

export function fieldSpecsFor(entity: Entity): FieldSpec[] {
	return entity === 'tracks' ? TRACK_FIELDS : ALBUM_FIELDS;
}

export function findFieldSpec(entity: Entity, field: Field): FieldSpec | undefined {
	return fieldSpecsFor(entity).find((s) => s.field === field);
}

export interface LeafNode {
	kind: 'leaf';
	field: Field;
	op: Operator;
	value: FilterValue;
	negate: boolean;
}

export function isLeaf(node: FilterNode): node is LeafNode {
	return node.kind === 'leaf';
}

/** Build an empty (no-op) leaf with sensible defaults for the given field. */
export function defaultLeafForField(spec: FieldSpec): LeafNode {
	let value: FilterValue;
	switch (spec.editor) {
		case 'text':
			value = { kind: 'text', value: '' };
			break;
		case 'tag':
		case 'album':
		case 'artist':
			value = { kind: 'id', value: '' };
			break;
		case 'intRange':
		case 'yearRange':
			value = { kind: 'intRange', min: null, max: null };
			break;
	}
	return {
		kind: 'leaf',
		field: spec.field,
		op: spec.defaultOp,
		value,
		negate: false,
	};
}
