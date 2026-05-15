<script lang="ts">
	import { getPlayer } from '$lib/player/player.svelte';

	let { class: className }: { class?: string } = $props();

	const player = getPlayer();
</script>

<div class={['flex h-4 w-5 items-end justify-between gap-[2px]', className]} aria-hidden="true">
	{#each player.levels as level, i (i)}
		<span class="w-[2px] flex-1 rounded-sm bg-primary" style:height="{Math.max(level, 0.08) * 100}%"
		></span>
	{/each}
</div>
