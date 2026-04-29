<script lang="ts">
	import { getPlayer } from '$lib/player/player.svelte';
	import { Button } from '$lib/components/ui/button';
	import PlayIcon from '@lucide/svelte/icons/play';
	import PauseIcon from '@lucide/svelte/icons/pause';
	import SkipBackIcon from '@lucide/svelte/icons/skip-back';
	import SkipForwardIcon from '@lucide/svelte/icons/skip-forward';
	import ListMusicIcon from '@lucide/svelte/icons/list-music';
	import Volume2Icon from '@lucide/svelte/icons/volume-2';
	import VolumeXIcon from '@lucide/svelte/icons/volume-x';
	import { formatMilliseconds } from '$lib/utils/date';
	import QueueDrawer from './queue-drawer.svelte';
	import { RangeSlider } from '$lib/components/ui/range-slider';

	const player = getPlayer();
	let audioEl = $state<HTMLAudioElement | null>(null);

	$effect(() => {
		if (!audioEl) return;
		// Depend on streamUrl so track switches re-issue play() against the new src;
		// changing `src` silently pauses the element without firing a pause event.
		// eslint-disable-next-line @typescript-eslint/no-unused-expressions
		player.streamUrl;
		if (player.isPlaying) {
			audioEl.play().catch(() => {
				player.isPlaying = false;
			});
		} else {
			audioEl.pause();
		}
	});

	function onSeekInput(value: number) {
		player.currentTime = value;
		if (audioEl) audioEl.currentTime = value;
	}
</script>

{#if player.currentTrack}
	<footer
		class="fixed inset-x-0 bottom-0 z-50 flex h-20 items-center gap-4 border-t bg-background px-4"
	>
		<audio
			bind:this={audioEl}
			src={player.streamUrl}
			bind:currentTime={player.currentTime}
			bind:duration={player.duration}
			bind:volume={player.volume}
			onplay={() => (player.isPlaying = true)}
			onpause={() => (player.isPlaying = false)}
			onended={() => player.onEnded()}
			preload="metadata"
		></audio>

		<div class="flex min-w-0 flex-1 items-center gap-3">
			{#if player.currentTrack.album?.coverUrl}
				<img src={player.currentTrack.album.coverUrl} class="size-12 shrink-0 rounded" alt="" />
			{/if}
			<div class="flex min-w-0 flex-col">
				<span class="truncate font-medium">{player.currentTrack.title}</span>
				<span class="truncate text-sm text-muted-foreground">
					{player.currentTrack.artist?.name ?? ''}
				</span>
			</div>
		</div>

		<div class="flex items-center gap-1">
			<Button
				size="icon"
				variant="ghost"
				onclick={() => player.prev()}
				disabled={!player.hasPrev}
				aria-label="Previous track"
			>
				<SkipBackIcon />
			</Button>
			<Button
				size="icon"
				onclick={() => player.toggle()}
				aria-label={player.isPlaying ? 'Pause' : 'Play'}
			>
				{#if player.isPlaying}
					<PauseIcon />
				{:else}
					<PlayIcon />
				{/if}
			</Button>
			<Button
				size="icon"
				variant="ghost"
				onclick={() => player.next()}
				disabled={!player.hasNext}
				aria-label="Next track"
			>
				<SkipForwardIcon />
			</Button>
		</div>

		<div class="flex flex-1 items-center justify-end gap-2">
			<p class="text-xs text-muted-foreground tabular-nums">
				<span class="w-12 text-right">
					{formatMilliseconds(player.currentTime * 1000)}
				</span>
				/
				<span class="w-12">
					{formatMilliseconds((player.duration || 0) * 1000)}
				</span>
			</p>

			<button
				onclick={() => player.toggleMute()}
				aria-label={player.volume === 0 ? 'Unmute' : 'Mute'}
				class="shrink-0 cursor-pointer text-muted-foreground transition-colors hover:text-foreground"
			>
				{#if player.volume === 0}
					<VolumeXIcon class="size-4" />
				{:else}
					<Volume2Icon class="size-4" />
				{/if}
			</button>
			<RangeSlider
				min={0}
				max={1}
				step={0.01}
				value={player.volume}
				oninput={(e) => (player.volume = +(e.currentTarget as HTMLInputElement).value)}
				aria-label="Volume"
				class="w-24"
			/>

			<Button
				size="icon"
				variant="ghost"
				onclick={() => player.toggleQueue()}
				aria-label="Toggle queue"
				aria-pressed={player.queueOpen}
			>
				<ListMusicIcon />
			</Button>
		</div>

		<!-- full-width seek bar pinned to top edge; outer div is the hitbox, inner is visual -->
		<div class="group absolute inset-x-0 top-0 h-4">
			<div class="absolute inset-x-0 top-0 h-px bg-muted transition-all duration-150 group-hover:h-[3px]">
				<div
					class="h-full bg-primary"
					style="width: {player.duration ? (player.currentTime / player.duration) * 100 : 0}%"
				></div>
				<div
					class="pointer-events-none absolute top-1/2 size-3 -translate-x-1/2 -translate-y-1/2 rounded-full bg-primary opacity-0 transition-opacity duration-150 group-hover:opacity-100"
					style="left: {player.duration ? (player.currentTime / player.duration) * 100 : 0}%"
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
	</footer>
	<QueueDrawer />
{/if}
