import '@tanstack/table-core';

declare module '@tanstack/table-core' {
	interface ColumnMeta {
		class?: string;
		mainColumn?: boolean;
		/** Hide this column by default until the user enables it via the column toggle. */
		defaultHidden?: boolean;
	}
}
