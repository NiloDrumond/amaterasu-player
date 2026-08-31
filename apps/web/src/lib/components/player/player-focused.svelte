<script lang="ts">
	import { getPlayer } from '$lib/player/player.svelte';
	import { createVolumeWheelHandler } from '$lib/player/volume-wheel';
	import { fade } from 'svelte/transition';
	import { formatMilliseconds } from '$lib/utils/date';
	import { cn } from '$lib/utils';
	import { useShortcut } from '$lib/shortcuts/shortcuts.svelte';
	import PlayerControls from './player-controls.svelte';
	import PlayerVolume from './player-volume.svelte';
	import PlayerSeek from './player-seek.svelte';
	import TrackActions from '$lib/components/tracks/track-actions.svelte';
	import FavoriteButton from '$lib/components/tracks/favorite-button.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Icons } from '$lib/components/ui/icons';

	const player = getPlayer();
	const onVolumeWheel = createVolumeWheelHandler(player);

	const IDLE_MS = 5000;
	let idle = $state(false);
	let timer: ReturnType<typeof setTimeout> | null = null;

	function resetIdle() {
		idle = false;
		if (timer) clearTimeout(timer);
		timer = setTimeout(() => (idle = true), IDLE_MS);
	}

	function clearIdle() {
		idle = false;
		if (timer) {
			clearTimeout(timer);
			timer = null;
		}
	}

	$effect(() => {
		if (!player.focusedOpen) {
			clearIdle();
			return;
		}
		resetIdle();
		return () => clearIdle();
	});

	useShortcut({
		id: 'focused.close',
		keys: 'Escape',
		when: () => player.focusedOpen,
		handler: () => player.closeFocused(),
	});

	function onBackdropClick(e: MouseEvent) {
		if (e.target === e.currentTarget) player.closeFocused();
	}
</script>

{#if player.focusedOpen && player.currentTrack}
	<div
		class={cn(
			'fixed inset-0 z-[60] flex flex-col items-center justify-center gap-8 bg-background p-8',
			idle && 'cursor-none',
		)}
		onmousemove={resetIdle}
		onpointerdown={resetIdle}
		onclick={onBackdropClick}
		onwheel={onVolumeWheel}
		role="presentation"
		transition:fade={{ duration: 150 }}
	>
		<Button
			size="icon"
			variant="ghost"
			class={cn(
				'absolute top-6 right-6 transition-opacity duration-500',
				idle && 'pointer-events-none opacity-0',
			)}
			onclick={() => player.closeFocused()}
			aria-label="Close focused view"
		>
			<Icons.Close />
		</Button>

		{#if player.currentTrack.album?.coverUrl}
			<img
				src={player.currentTrack.album.coverUrl}
				alt=""
				class="aspect-square max-h-[60vh] w-auto rounded-lg object-cover shadow-2xl"
			/>
		{:else}
			<div class="aspect-square h-[60vh] rounded-lg bg-muted shadow-2xl"></div>
		{/if}

		<div class="flex flex-col items-center gap-1 text-center">
			<div class="flex items-center gap-2">
				<span class="text-3xl font-semibold">{player.currentTrack.title}</span>
				<FavoriteButton track={player.currentTrack} class="size-9 shrink-0" />
			</div>
			{#if player.currentTrack.artist}
				<span class="text-lg text-muted-foreground">
					{player.currentTrack.artist.name}
				</span>
			{/if}
		</div>

		<div class="flex w-full max-w-2xl flex-col gap-2">
			<PlayerSeek variant="expanded" />
			<div
				class={cn(
					'flex justify-between text-xs text-muted-foreground tabular-nums transition-opacity duration-500',
					idle && 'opacity-0',
				)}
			>
				<span>{formatMilliseconds(player.currentTime * 1000)}</span>
				<span>{formatMilliseconds((player.duration || 0) * 1000)}</span>
			</div>
		</div>

		<div
			class={cn(
				'flex flex-col items-center gap-3 transition-opacity duration-500',
				idle && 'pointer-events-none opacity-0',
			)}
		>
			<div class="flex items-center gap-2">
				<PlayerControls />
			</div>
			<div class="flex items-center gap-2">
				<PlayerVolume />
				{#if player.currentTrack}
					<TrackActions track={player.currentTrack} />
				{/if}
			</div>
		</div>
	</div>
{/if}
