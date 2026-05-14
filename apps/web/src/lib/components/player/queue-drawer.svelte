<script lang="ts">
	import { getPlayer } from '$lib/player/player.svelte';
	import { Button } from '$lib/components/ui/button';
	import { draggable, droppable, type DragDropState } from '@thisux/sveltednd';
	import type { TrackResponse } from '$lib/bindings/response/track/track-response';
	import { fly } from 'svelte/transition';
	import { Icons } from '$lib/components/ui/icons';

	const player = getPlayer();

	function onDrop(state: DragDropState<TrackResponse>) {
		const from = parseInt(state.sourceContainer);
		const targetRaw = state.targetContainer;
		if (targetRaw === null || Number.isNaN(from)) return;
		let to = parseInt(targetRaw);
		if (Number.isNaN(to)) return;
		if (state.dropPosition === 'after') to++;
		const adjusted = from < to ? to - 1 : to;
		if (from !== adjusted) player.reorder(from, adjusted);
	}

	function onRowClick(e: MouseEvent, i: number) {
		const t = e.target as HTMLElement | null;
		if (t?.closest('button')) return;
		player.jumpTo(i);
	}

	function onRowKeydown(e: KeyboardEvent, i: number) {
		if (e.key !== 'Enter' && e.key !== ' ') return;
		const t = e.target as HTMLElement | null;
		if (t?.closest('button')) return;
		e.preventDefault();
		player.jumpTo(i);
	}
</script>

{#if player.queueOpen}
	<aside
		class="fixed top-0 right-0 bottom-20 z-40 flex w-96 flex-col border-l bg-background"
		transition:fly={{ x: 400, duration: 200 }}
	>
		<header class="flex h-12 shrink-0 items-center justify-between border-b px-4">
			<span class="font-medium">Queue</span>
			<Button
				size="icon"
				variant="ghost"
				onclick={() => player.toggleQueue()}
				aria-label="Close queue"
			>
				<Icons.Close />
			</Button>
		</header>

		{#if player.queue.length === 0}
			<div class="flex flex-1 items-center justify-center text-sm text-muted-foreground">
				Queue is empty
			</div>
		{:else}
			<div class="flex-1 overflow-y-auto" role="list">
				{#each player.queue as track, i (track.id + '-' + i)}
					<div
						use:draggable={{ container: i.toString(), dragData: track }}
						use:droppable={{ container: i.toString(), callbacks: { onDrop } }}
						class={[
							'flex cursor-pointer items-center gap-2 px-2 py-2 hover:bg-accent/50 focus-visible:ring-2 focus-visible:ring-ring focus-visible:outline-none',
							i === player.index && 'bg-accent',
						]}
						role="button"
						tabindex="0"
						onclick={(e) => onRowClick(e, i)}
						onkeydown={(e) => onRowKeydown(e, i)}
					>
						<Icons.Grip class="size-4 shrink-0 text-muted-foreground" />
						{#if track.album?.coverUrl}
							<img src={track.album.coverUrl} alt="" class="size-10 shrink-0 rounded" />
						{:else}
							<div class="size-10 shrink-0 rounded bg-muted"></div>
						{/if}
						<div class="flex min-w-0 flex-1 flex-col">
							<span class="truncate text-sm font-medium">{track.title}</span>
							<span class="truncate text-xs text-muted-foreground">{track.artist?.name ?? ''}</span>
						</div>
						<Button
							size="icon"
							variant="ghost"
							onclick={() => player.removeAt(i)}
							aria-label="Remove"
						>
							<Icons.Close />
						</Button>
					</div>
				{/each}
			</div>
		{/if}
	</aside>
{/if}
