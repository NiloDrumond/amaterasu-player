<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Icons } from '$lib/components/ui/icons';
	import { cn } from '$lib/utils';

	let {
		active,
		onShuffle,
	}: {
		active: boolean;
		onShuffle: () => void;
	} = $props();
</script>

<Button
	variant="ghost"
	size="icon"
	aria-pressed={active}
	title={active ? 'Re-shuffle' : 'Shuffle'}
	onclick={onShuffle}
	class={cn(active && 'bg-muted text-primary hover:text-primary')}
>
	<Icons.Shuffle class="size-4" />
</Button>
