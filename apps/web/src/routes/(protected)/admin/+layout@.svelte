<script lang="ts">
	import AdminSidebar from '$lib/components/admin/admin-sidebar.svelte';
	import * as Sidebar from '$lib/components/ui/sidebar';

	let { data, children } = $props();
</script>

<Sidebar.Provider>
	<AdminSidebar user={data.user} />
	<Sidebar.Inset>
		{@render children()}
	</Sidebar.Inset>
</Sidebar.Provider>
