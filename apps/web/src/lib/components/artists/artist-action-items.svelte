<script lang="ts">
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';

	let { id }: { id: string } = $props();
</script>

<DropdownMenu.Group>
	<DropdownMenu.Label>Actions</DropdownMenu.Label>
	<DropdownMenu.Item onclick={() => navigator.clipboard.writeText(id)}>Copy ID</DropdownMenu.Item>
</DropdownMenu.Group>
