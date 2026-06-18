<script lang="ts">
	import Badge from '$lib/components/ui/badge/badge.svelte';

	let { role }: { role: string } = $props();
</script>

<Badge variant={role === 'admin' ? 'default' : 'secondary'}>{role}</Badge>
