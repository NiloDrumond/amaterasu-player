<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import { Button } from '$lib/components/ui/button';
	import { Icons } from '$lib/components/ui/icons';
	import {
		TAG_COLOR_NAMES,
		TAG_COLOR_TONES,
		resolveTagColor,
		tagForegroundColor,
	} from './tag-color';

	let {
		value = $bindable(''),
	}: {
		value?: string;
	} = $props();

	let open = $state(false);

	function pick(token: string) {
		value = value === token ? '' : token;
		open = false;
	}

	const previewBg = $derived(resolveTagColor(value));
	const previewFg = $derived(tagForegroundColor(value));

	const halfPoint = $derived(Math.ceil(TAG_COLOR_NAMES.length / 2));
	const leftColumn = $derived(TAG_COLOR_NAMES.slice(0, halfPoint));
	const rightColumn = $derived(TAG_COLOR_NAMES.slice(halfPoint));
</script>

<Dialog.Root bind:open>
	<Dialog.Trigger>
		{#snippet child({ props })}
			<button
				{...props}
				type="button"
				class="flex h-9 w-full items-center justify-between rounded-md border border-input px-3 py-1 text-sm transition-colors hover:bg-muted"
				style:background-color={previewBg ?? undefined}
				style:color={value ? previewFg : undefined}
			>
				<span class="font-medium">{value || 'No color'}</span>
				{#if !value}
					<span class="text-xs text-muted-foreground">Click to choose</span>
				{/if}
			</button>
		{/snippet}
	</Dialog.Trigger>
	<Dialog.Content class="sm:max-w-lg">
		<Dialog.Header>
			<Dialog.Title>Pick a color</Dialog.Title>
			<Dialog.Description>
				Choose a color and tone, or clear to leave the tag uncolored.
			</Dialog.Description>
		</Dialog.Header>

		<div class="flex flex-col gap-4">
			<button
				type="button"
				class="flex items-center gap-2 self-start rounded-sm border border-border px-2 py-1 text-xs text-muted-foreground transition-colors hover:bg-muted"
				onclick={() => {
					value = '';
					open = false;
				}}
			>
				<Icons.Close class="size-3" />
				Clear color
			</button>
			<div class="flex justify-center gap-4">
				{#each [leftColumn, rightColumn] as column, columnIndex (columnIndex)}
					<div class="flex flex-col gap-1 {columnIndex === 0 ? 'border-r pr-4' : ''}">
						{#each column as name (name)}
							<div class="flex gap-1">
								{#each TAG_COLOR_TONES as tone (tone)}
									{@const token = `${name}-${tone}`}
									{@const selected = value === token}
									<button
										type="button"
										class="relative flex size-6 shrink-0 items-center justify-center rounded-sm ring-offset-background transition-transform hover:scale-110 focus-visible:ring-2 focus-visible:ring-ring focus-visible:outline-none"
										style:background-color={resolveTagColor(token)}
										aria-label={token}
										aria-pressed={selected}
										onclick={() => pick(token)}
									>
										{#if selected}
											<Icons.Check class="size-3.5" color={tone <= 400 ? 'black' : 'white'} />
										{/if}
									</button>
								{/each}
							</div>
						{/each}
					</div>
				{/each}
			</div>
		</div>

		<Dialog.Footer>
			<Dialog.Close>
				{#snippet child({ props })}
					<Button {...props} variant="ghost" type="button">Done</Button>
				{/snippet}
			</Dialog.Close>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
