<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Icons } from '$lib/components/ui/icons';
	import { RangeSlider } from '$lib/components/ui/range-slider';
	import { getPlayer } from '$lib/player/player.svelte';

	const player = getPlayer();
</script>

<Button
	size="icon"
	variant="ghost"
	onclick={() => player.toggleMute()}
	aria-label={player.volume === 0 ? 'Unmute' : 'Mute'}
>
	{#if player.volume === 0}
		<Icons.VolumeMute class="size-4" />
	{:else}
		<Icons.Volume class="size-4" />
	{/if}
</Button>
<RangeSlider
	min={0}
	max={1}
	step={0.01}
	value={player.volume}
	oninput={(e) => (player.volume = +(e.currentTarget as HTMLInputElement).value)}
	aria-label="Volume"
	class="w-24"
/>
