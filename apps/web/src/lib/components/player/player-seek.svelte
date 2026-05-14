<script lang="ts">
	import { getPlayer } from '$lib/player/player.svelte';

	type Props = {
		variant?: 'bar' | 'expanded';
		class?: string;
	};

	let { variant = 'bar', class: className = '' }: Props = $props();

	const player = getPlayer();

	const percent = $derived(player.duration ? (player.currentTime / player.duration) * 100 : 0);

	function onSeekInput(value: number) {
		player.currentTime = value;
	}
</script>

{#if variant === 'bar'}
	<div class="group absolute inset-x-0 top-0 h-4 {className}" data-no-focus>
		<div
			class="absolute inset-x-0 top-0 h-px bg-muted transition-all duration-150 group-hover:h-0.75"
		>
			<div class="h-full bg-primary" style="width: {percent}%"></div>
			<div
				class="pointer-events-none absolute top-1/2 size-3 -translate-x-1/2 -translate-y-1/2 rounded-full bg-primary opacity-0 transition-opacity duration-150 group-hover:opacity-100"
				style="left: {percent}%"
			></div>
		</div>
		<input
			type="range"
			min="0"
			max={player.duration || 0}
			step="0.1"
			value={player.currentTime}
			oninput={(e) => onSeekInput(+e.currentTarget.value)}
			aria-label="Seek"
			class="absolute inset-0 h-full w-full cursor-pointer opacity-0"
		/>
	</div>
{:else}
	<div class="group relative h-2 w-full {className}">
		<div class="absolute inset-x-0 top-1/2 h-1 -translate-y-1/2 rounded-full bg-muted">
			<div class="h-full rounded-full bg-primary" style="width: {percent}%"></div>
			<div
				class="pointer-events-none absolute top-1/2 size-3.5 -translate-x-1/2 -translate-y-1/2 rounded-full bg-primary opacity-0 shadow transition-opacity duration-150 group-hover:opacity-100"
				style="left: {percent}%"
			></div>
		</div>
		<input
			type="range"
			min="0"
			max={player.duration || 0}
			step="0.1"
			value={player.currentTime}
			oninput={(e) => onSeekInput(+e.currentTarget.value)}
			aria-label="Seek"
			class="absolute inset-0 h-full w-full cursor-pointer opacity-0"
		/>
	</div>
{/if}
