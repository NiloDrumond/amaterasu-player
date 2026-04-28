import '@tanstack/table-core';

declare module '@tanstack/table-core' {
	interface ColumnMeta {
		class?: string;
		mainColumn?: boolean;
	}
}
