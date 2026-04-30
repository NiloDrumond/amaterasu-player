<script lang="ts">
	import { resolveTagColor } from './tag-color';

	let { color }: { color: string | null } = $props();
	const resolved = $derived(resolveTagColor(color));
</script>

<div
	class="size-4 rounded-full border border-border"
	style:background-color={resolved ?? 'transparent'}
></div>
