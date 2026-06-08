<script lang="ts">
	import { page } from '$app/state';
	import { Button } from '$lib/components/ui/button';
</script>

<div class="flex min-h-[60vh] flex-col items-center justify-center gap-3 text-center">
	<p class="text-4xl font-semibold tabular-nums">{page.status}</p>
	<p class="max-w-md text-sm text-muted-foreground">
		{page.error?.message ?? 'Something went wrong.'}
	</p>
	<Button href="/" variant="outline" size="sm">Go home</Button>
</div>
