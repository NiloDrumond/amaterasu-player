<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Icons } from '$lib/components/ui/icons';
	import { getPlayer } from '$lib/player/player.svelte';
	import { cn } from '$lib/utils';

	const player = getPlayer();
</script>

<Button
	size="icon"
	variant="ghost"
	class={cn(player.shuffleEnabled ? 'text-foreground' : 'text-muted-foreground')}
	onclick={() => player.toggleShuffle()}
	aria-label="Shuffle"
	aria-pressed={player.shuffleEnabled}
>
	<Icons.Shuffle weight={player.shuffleEnabled ? 'bold' : 'regular'} />
</Button>
<Button
	size="icon"
	variant="ghost"
	onclick={() => player.prev()}
	disabled={!player.hasPrev}
	aria-label="Previous track"
>
	<Icons.SkipBack />
</Button>
<Button
	size="icon"
	variant="ghost"
	onclick={() => player.toggle()}
	aria-label={player.isPlaying ? 'Pause' : 'Play'}
>
	{#if player.isPlaying}
		<Icons.Pause />
	{:else}
		<Icons.Play />
	{/if}
</Button>
<Button
	size="icon"
	variant="ghost"
	onclick={() => player.next()}
	disabled={!player.hasNext}
	aria-label="Next track"
>
	<Icons.SkipForward />
</Button>
<Button
	size="icon"
	variant={player.repeatMode === 'off' ? 'ghost' : 'secondary'}
	onclick={() => player.cycleRepeatMode()}
	aria-label={player.repeatMode === 'one'
		? 'Repeat one'
		: player.repeatMode === 'all'
			? 'Repeat all'
			: 'Repeat off'}
>
	{#if player.repeatMode === 'one'}
		<Icons.RepeatOne weight="bold" />
	{:else if player.repeatMode === 'all'}
		<Icons.Repeat weight="bold" />
	{:else}
		<Icons.Repeat />
	{/if}
</Button>
