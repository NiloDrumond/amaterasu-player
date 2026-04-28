<script lang="ts">
	import { getPlayer } from '$lib/player/player.svelte';
	import { Button } from '$lib/components/ui/button';
	import PlayIcon from '@lucide/svelte/icons/play';
	import PauseIcon from '@lucide/svelte/icons/pause';
	import SkipBackIcon from '@lucide/svelte/icons/skip-back';
	import SkipForwardIcon from '@lucide/svelte/icons/skip-forward';
	import ListMusicIcon from '@lucide/svelte/icons/list-music';
	import { formatMilliseconds } from '$lib/utils/date';
	import QueueDrawer from './queue-drawer.svelte';

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

		<div class="flex flex-1 items-center gap-2">
			<span class="w-12 text-right text-xs text-muted-foreground tabular-nums">
				{formatMilliseconds(player.currentTime * 1000)}
			</span>
			<input
				type="range"
				min="0"
				max={player.duration || 0}
				step="0.1"
				value={player.currentTime}
				oninput={(e) => onSeekInput(+e.currentTarget.value)}
				class="flex-1"
				aria-label="Seek"
			/>
			<span class="w-12 text-xs text-muted-foreground tabular-nums">
				{formatMilliseconds((player.duration || 0) * 1000)}
			</span>
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
	</footer>
	<QueueDrawer />
{/if}
