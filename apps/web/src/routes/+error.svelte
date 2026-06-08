<script lang="ts">
	import { page } from '$app/state';
	import { Button } from '$lib/components/ui/button';
</script>

<div class="flex min-h-[60vh] flex-col items-center justify-center gap-3 text-center">
	<p class="text-4xl font-semibold tabular-nums">{page.status}</p>
	<p class="text-muted-foreground max-w-md text-sm">
		{page.error?.message ?? 'Something went wrong.'}
	</p>
	<Button href="/" variant="outline" size="sm">Go home</Button>
</div>
