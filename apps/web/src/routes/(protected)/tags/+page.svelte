<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import DataTable from '$lib/components/ui/data-table/data-table.svelte';
	import { Button } from '$lib/components/ui/button';
	import { tagsColumns } from '$lib/components/tags/columns.js';
	import TagDialog from '$lib/components/tags/tag-dialog.svelte';
	import TagRowContextMenu from '$lib/components/tags/tag-row-context-menu.svelte';
	import PlusIcon from '@lucide/svelte/icons/plus';
	import type { TagResponse } from '$lib/bindings/response/tag/tag-response';

	let { data } = $props();

	let dialogOpen = $state(false);
	let editing = $state<TagResponse | null>(null);

	function openCreate() {
		editing = null;
		dialogOpen = true;
	}

	function openEdit(tag: TagResponse) {
		editing = tag;
		dialogOpen = true;
	}

	const columns = $derived(tagsColumns(openEdit, () => invalidateAll()));
</script>

{#if data.error}
	<h1>Error</h1>
	<p>{data.error.message}</p>
{:else}
	<div class="flex flex-col p-4">
		<div class="mb-4 flex items-center justify-between">
			<h1 class="text-xl font-bold tracking-widest uppercase">TAGS</h1>
			<Button class="gap-2" onclick={openCreate}>
				<PlusIcon class="size-4" />
				New Tag
			</Button>
		</div>

		<DataTable data={data.tags ?? []} {columns} onRowClick={(row) => openEdit(row)}>
			{#snippet rowContextMenu({ row, trigger })}
				<TagRowContextMenu
					tag={row}
					onEdit={openEdit}
					onDeleted={() => invalidateAll()}
					{trigger}
				/>
			{/snippet}
		</DataTable>
	</div>

	<TagDialog bind:open={dialogOpen} tag={editing} onSaved={() => invalidateAll()} />
{/if}
