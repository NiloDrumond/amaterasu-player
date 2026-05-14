<script lang="ts">
	import type { HTMLInputAttributes } from 'svelte/elements';
	import { cn } from '$lib/utils.js';

	type Props = Omit<HTMLInputAttributes, 'type'> & {
		value?: number;
		min?: number;
		max?: number;
	};

	let { value = 0, min = 0, max = 1, class: className, ...rest }: Props = $props();

	const progress = $derived(max > min ? ((value - min) / (max - min)) * 100 : 0);
</script>

<input
	type="range"
	{min}
	{max}
	{...rest}
	{value}
	class={cn('range-slider', className)}
	style="--progress: {progress}%"
/>

<style>
	.range-slider {
		-webkit-appearance: none;
		appearance: none;
		background: transparent;
		cursor: pointer;
	}

	/* track */
	.range-slider::-webkit-slider-runnable-track {
		height: 3px;
		border-radius: 9999px;
		background: linear-gradient(
			to right,
			var(--primary) var(--progress),
			var(--muted) var(--progress)
		);
	}

	.range-slider::-moz-range-track {
		height: 3px;
		border-radius: 9999px;
		background: var(--muted);
	}

	.range-slider::-moz-range-progress {
		height: 3px;
		border-radius: 9999px;
		background: var(--primary);
	}

	/* thumb */
	.range-slider::-webkit-slider-thumb {
		-webkit-appearance: none;
		appearance: none;
		width: 10px;
		height: 10px;
		border-radius: 50%;
		background: var(--primary);
		margin-top: -3.5px;
		cursor: pointer;
	}

	.range-slider::-moz-range-thumb {
		width: 10px;
		height: 10px;
		border-radius: 50%;
		background: var(--primary);
		border: none;
		cursor: pointer;
	}
</style>
