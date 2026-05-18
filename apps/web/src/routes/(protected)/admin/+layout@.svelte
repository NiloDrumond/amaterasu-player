<script lang="ts">
	import AdminSidebar from '$lib/components/admin/admin-sidebar.svelte';
	import * as Sidebar from '$lib/components/ui/sidebar';
	import { setPlayer } from '$lib/player/player.svelte';

	let { data, children } = $props();

	setPlayer();
</script>

<Sidebar.Provider>
	<AdminSidebar user={data.user} reviewCounts={data.reviewCounts ?? null} />
	<Sidebar.Inset>
		{@render children()}
	</Sidebar.Inset>
</Sidebar.Provider>
