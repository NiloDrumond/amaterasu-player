<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { scanLibrary } from '$lib/services/admin-service';
	import { toast } from 'svelte-sonner';

	async function handleScan() {
		const { error, status } = await scanLibrary(fetch);
		if (!error) toast.success('Library scan started');
		else if (status === 409) toast.info('A scan is already running');
		else toast.error('Failed to start scan', { description: error });
	}
</script>

<div class="mx-auto max-w-2xl min-w-md space-y-6 p-6">
	<header>
		<h1>Admin</h1>
		<p class="text-sm text-muted-foreground">
			Edit metadata for tracks, albums, and artists. Edits are protected from being overwritten by
			rescans.
		</p>
	</header>

	<section class="space-y-2">
		<h2>Library</h2>
		<div class="flex gap-2">
			<Button onclick={handleScan}>Scan library</Button>
			<Button variant="outline" href="/admin/tracks/deleted">Deleted tracks</Button>
		</div>
	</section>

	<section class="space-y-2">
		<h2>Help</h2>
		<ul class="list-inside list-disc space-y-1 text-sm text-muted-foreground">
			<li>Open an entity's admin page from its right-click context menu.</li>
			<li>Any edit "locks" the row so future scans don't overwrite it.</li>
			<li>"Force rescan" clears the lock so file-tag values come back on the next scan.</li>
		</ul>
	</section>
</div>
