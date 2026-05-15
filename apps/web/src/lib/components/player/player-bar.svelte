<script lang="ts">
	import { getPlayer } from '$lib/player/player.svelte';
	import { Button } from '$lib/components/ui/button';
	import { formatMilliseconds } from '$lib/utils/date';
	import QueueDrawer from './queue-drawer.svelte';
	import PlayerFocused from './player-focused.svelte';
	import PlayerControls from './player-controls.svelte';
	import PlayerVolume from './player-volume.svelte';
	import PlayerSeek from './player-seek.svelte';
	import TrackActions from '$lib/components/tracks/track-actions.svelte';
	import { Icons } from '$lib/components/ui/icons';

	const player = getPlayer();
	let audioEl = $state<HTMLAudioElement | null>(null);

	$effect(() => {
		if (!audioEl) return;
		player.attachAudio(audioEl);
	});

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

	function onBarClick(e: MouseEvent) {
		const target = e.target as HTMLElement | null;
		if (!target) return;
		if (target.closest('button, input, a, [role="menuitem"], [data-no-focus]')) return;
		player.openFocused();
	}
</script>

{#if player.currentTrack}
	<footer
		class="fixed inset-x-0 bottom-0 z-50 flex h-20 cursor-pointer items-center gap-4 border-t bg-background px-4"
		onclick={onBarClick}
		role="presentation"
	>
		<audio
			bind:this={audioEl}
			src={player.streamUrl}
			bind:currentTime={player.currentTime}
			bind:duration={player.duration}
			bind:volume={player.volume}
			onplay={() => {
				player.isPlaying = true;
				player.resumeAudioContext();
			}}
			onpause={() => (player.isPlaying = false)}
			ontimeupdate={() => player.maybeScrobble()}
			onended={() => player.onEnded()}
			preload="metadata"
		></audio>

		<div class="flex min-w-0 flex-1 items-center gap-3">
			{#if player.currentTrack.album?.coverUrl && player.currentTrack.album}
				<a href="/albums/{player.currentTrack.album.id}" class="shrink-0" data-no-focus>
					<img src={player.currentTrack.album.coverUrl} class="size-12 rounded" alt="" />
				</a>
			{/if}
			<div class="flex min-w-0 flex-col">
				<span class="truncate font-medium">{player.currentTrack.title}</span>
				{#if player.currentTrack.artist}
					<a
						href="/artists/{player.currentTrack.artist.id}"
						class="w-fit truncate text-sm text-muted-foreground transition-colors hover:text-foreground"
						data-no-focus
					>
						{player.currentTrack.artist.name}
					</a>
				{/if}
			</div>
		</div>

		<div class="flex items-center gap-1">
			<PlayerControls />
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

			<PlayerVolume />

			{#if player.currentTrack}
				<div data-no-focus>
					<TrackActions track={player.currentTrack} />
				</div>
			{/if}

			<Button
				size="icon"
				variant="ghost"
				onclick={() => player.toggleQueue()}
				aria-label="Toggle queue"
				aria-pressed={player.queueOpen}
			>
				<Icons.Playlist />
			</Button>
		</div>

		<PlayerSeek variant="bar" />
	</footer>
	<QueueDrawer />
	<PlayerFocused />
{/if}
