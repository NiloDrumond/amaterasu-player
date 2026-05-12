<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import EntityPicker from './entity-picker.svelte';
	import {
		searchArtists,
		searchAlbums,
		getAdminArtist,
		mergeAdminArtist,
		mergeAdminAlbum,
	} from '$lib/services/admin-service';
	import type { AdminArtistResponse } from '$lib/bindings/response/admin/admin-artist-response';
	import type { AdminAlbumResponse } from '$lib/bindings/response/admin/admin-album-response';
	import { toast } from 'svelte-sonner';
	import { invalidateAll } from '$app/navigation';

	type FieldChoice = 'target' | 'source' | 'custom';
	type FieldKey = 'name' | 'sortName' | 'title' | 'sortTitle' | 'artistId' | 'date' | 'coverPath';

	type Props =
		| {
				kind: 'artist';
				open: boolean;
				target: AdminArtistResponse;
				targetArtist?: never;
		  }
		| {
				kind: 'album';
				open: boolean;
				target: AdminAlbumResponse;
				targetArtist: AdminArtistResponse | null;
		  };

	let { kind, open = $bindable(), target, targetArtist }: Props = $props();

	let step = $state<'pick' | 'resolve'>('pick');
	let source = $state<AdminArtistResponse | AdminAlbumResponse | null>(null);
	let pickerValue = $state<AdminArtistResponse | AdminAlbumResponse | null>(null);
	let sourceArtist = $state<AdminArtistResponse | null>(null);
	let saving = $state(false);

	$effect(() => {
		if (pickerValue && step === 'pick') {
			pickSource(pickerValue);
		}
	});

	// Per-field state: chosen radio + custom text.
	let choices = $state<Record<FieldKey, FieldChoice>>({
		name: 'target',
		sortName: 'target',
		title: 'target',
		sortTitle: 'target',
		artistId: 'target',
		date: 'target',
		coverPath: 'target',
	});
	let customs = $state<Record<FieldKey, string>>({
		name: '',
		sortName: '',
		title: '',
		sortTitle: '',
		artistId: '',
		date: '',
		coverPath: '',
	});

	function resetState() {
		step = 'pick';
		source = null;
		pickerValue = null;
		sourceArtist = null;
		choices = {
			name: 'target',
			sortName: 'target',
			title: 'target',
			sortTitle: 'target',
			artistId: 'target',
			date: 'target',
			coverPath: 'target',
		};
		customs = {
			name: '',
			sortName: '',
			title: '',
			sortTitle: '',
			artistId: '',
			date: '',
			coverPath: '',
		};
	}

	$effect(() => {
		if (!open) resetState();
	});

	async function searchSource(q: string) {
		if (kind === 'artist') {
			const { data } = await searchArtists(fetch, q);
			return (data ?? []).filter((a) => a.id !== target.id);
		} else {
			const { data } = await searchAlbums(fetch, q);
			return (data ?? []).filter((a) => a.id !== target.id);
		}
	}

	async function pickSource(picked: AdminArtistResponse | AdminAlbumResponse) {
		source = picked;
		// Default each field's choice: take source if target is empty, target otherwise.
		const fieldsForKind: FieldKey[] =
			kind === 'artist'
				? ['name', 'sortName']
				: ['title', 'sortTitle', 'artistId', 'date', 'coverPath'];
		for (const f of fieldsForKind) {
			const t = (target as Record<string, unknown>)[f];
			const s = (picked as Record<string, unknown>)[f];
			if (t == null || t === '') {
				choices[f] = s == null || s === '' ? 'target' : 'source';
			} else {
				choices[f] = 'target';
			}
		}
		// For album merges, resolve the source's display artist so the picker
		// shows a name not a UUID.
		if (kind === 'album') {
			const aid = (picked as AdminAlbumResponse).artistId;
			if (aid) {
				const { data } = await getAdminArtist(fetch, aid);
				sourceArtist = data ?? null;
			} else {
				sourceArtist = null;
			}
		}
		step = 'resolve';
	}

	function fieldValue(entity: Record<string, unknown> | null, key: FieldKey): string {
		if (!entity) return '';
		const v = entity[key];
		return v == null ? '' : String(v);
	}

	function resolved(key: FieldKey): string {
		switch (choices[key]) {
			case 'target':
				return fieldValue(target as unknown as Record<string, unknown>, key);
			case 'source':
				return fieldValue(source as unknown as Record<string, unknown>, key);
			case 'custom':
				return customs[key];
		}
	}

	// Display labels for entity references (artistId).
	function artistLabel(side: 'target' | 'source'): string {
		if (kind !== 'album') return '';
		const a = side === 'target' ? targetArtist : sourceArtist;
		if (!a) {
			const id =
				side === 'target'
					? (target as AdminAlbumResponse).artistId
					: (source as AdminAlbumResponse | null)?.artistId;
			return id ? id : '(none)';
		}
		return a.name;
	}

	async function submit() {
		if (!source) return;
		saving = true;
		try {
			if (kind === 'artist') {
				const { data, error, status } = await mergeAdminArtist(fetch, target.id, {
					sourceId: source.id,
					name: resolved('name'),
					sortName: resolved('sortName'),
				});
				if (error) {
					if (status === 409) toast.error('Cannot merge', { description: error });
					else toast.error('Merge failed', { description: error });
				} else if (data) {
					toast.success('Artists merged');
					open = false;
					await invalidateAll();
				}
			} else {
				const artistIdRaw = resolved('artistId');
				const dateRaw = resolved('date');
				const coverRaw = resolved('coverPath');
				const { data, error, status } = await mergeAdminAlbum(fetch, target.id, {
					sourceId: source.id,
					title: resolved('title'),
					sortTitle: resolved('sortTitle'),
					artistId: artistIdRaw === '' ? null : artistIdRaw,
					date: dateRaw === '' ? null : dateRaw,
					coverPath: coverRaw === '' ? null : coverRaw,
				});
				if (error) {
					if (status === 409) toast.error('Cannot merge', { description: error });
					else toast.error('Merge failed', { description: error });
				} else if (data) {
					toast.success('Albums merged');
					open = false;
					await invalidateAll();
				}
			}
		} finally {
			saving = false;
		}
	}

	const artistFields: { key: FieldKey; label: string }[] = [
		{ key: 'name', label: 'Name' },
		{ key: 'sortName', label: 'Sort name' },
	];
	const albumFields: { key: FieldKey; label: string }[] = [
		{ key: 'title', label: 'Title' },
		{ key: 'sortTitle', label: 'Sort title' },
		{ key: 'artistId', label: 'Album artist' },
		{ key: 'date', label: 'Date' },
		{ key: 'coverPath', label: 'Cover' },
	];
	const fields = $derived(kind === 'artist' ? artistFields : albumFields);

	function displayValue(entity: 'target' | 'source', key: FieldKey): string {
		if (key === 'artistId') return artistLabel(entity);
		return fieldValue(
			entity === 'target'
				? (target as unknown as Record<string, unknown>)
				: (source as unknown as Record<string, unknown>),
			key,
		);
	}
</script>

<Dialog.Root bind:open>
	<Dialog.Content class="sm:max-w-2xl">
		<Dialog.Header>
			<Dialog.Title>Merge {kind}</Dialog.Title>
			<Dialog.Description>
				{#if step === 'pick'}
					Pick the {kind} to merge <span class="font-medium">into</span> this one. The picked
					{kind}
					will be deleted and its tracks {kind === 'artist' ? 'and albums ' : ''}re-pointed here.
				{:else}
					Choose which value to keep for each field, then confirm.
				{/if}
			</Dialog.Description>
		</Dialog.Header>

		{#if step === 'pick'}
			<EntityPicker
				label={kind === 'artist' ? 'Other artist to absorb' : 'Other album to absorb'}
				placeholder="Search…"
				bind:value={pickerValue}
				formatLabel={(item: AdminArtistResponse | AdminAlbumResponse) =>
					'name' in item ? item.name : item.title}
				search={searchSource}
				canCreate={false}
			/>
			<div class="text-xs text-muted-foreground">
				After picking, you'll resolve any field conflicts before the merge runs.
			</div>
		{:else if source}
			<div class="space-y-3">
				<table class="w-full text-sm">
					<thead>
						<tr class="text-left text-xs text-muted-foreground">
							<th class="pb-1 font-normal">Field</th>
							<th class="pb-1 font-normal">Keep this</th>
							<th class="pb-1 font-normal">Use other</th>
							<th class="pb-1 font-normal">Custom</th>
						</tr>
					</thead>
					<tbody>
						{#each fields as { key, label } (key)}
							{@const isCover = key === 'coverPath'}
							{@const isArtist = key === 'artistId'}
							{@const t = displayValue('target', key)}
							{@const s = displayValue('source', key)}
							{@const same = t === s && !isCover}
							<tr class="border-t" class:opacity-50={same}>
								<td class="py-2 pr-2 font-medium">{label}</td>
								<td class="py-2 pr-2">
									<label class="flex cursor-pointer items-start gap-2">
										<input
											type="radio"
											name={`merge-${key}`}
											value="target"
											bind:group={choices[key]}
											class="mt-1"
										/>
										{#if isCover}
											{@const url = (target as AdminAlbumResponse).coverUrl}
											{#if url}
												<img src={url} alt="" class="h-16 w-16 rounded border object-cover" />
											{:else}
												<span class="text-muted-foreground">—</span>
											{/if}
										{:else}
											<span class="break-words">{t || '—'}</span>
										{/if}
									</label>
								</td>
								<td class="py-2 pr-2">
									<label class="flex cursor-pointer items-start gap-2">
										<input
											type="radio"
											name={`merge-${key}`}
											value="source"
											bind:group={choices[key]}
											class="mt-1"
										/>
										{#if isCover}
											{@const url = (source as AdminAlbumResponse | null)?.coverUrl}
											{#if url}
												<img src={url} alt="" class="h-16 w-16 rounded border object-cover" />
											{:else}
												<span class="text-muted-foreground">—</span>
											{/if}
										{:else}
											<span class="break-words">{s || '—'}</span>
										{/if}
									</label>
								</td>
								<td class="py-2">
									{#if !isCover && !isArtist}
										<label class="flex cursor-pointer items-start gap-2">
											<input
												type="radio"
												name={`merge-${key}`}
												value="custom"
												bind:group={choices[key]}
												class="mt-1"
											/>
											<Input
												class="h-7 text-xs"
												bind:value={customs[key]}
												onfocus={() => (choices[key] = 'custom')}
											/>
										</label>
									{/if}
								</td>
							</tr>
						{/each}
					</tbody>
				</table>

				<div class="rounded-md border bg-muted/40 p-3 text-xs">
					<div class="mb-1 font-medium tracking-wide text-muted-foreground uppercase">
						Merged result
					</div>
					{#each fields as { key, label } (key)}
						<div class="flex items-center justify-between gap-4 py-0.5">
							<span class="text-muted-foreground">{label}</span>
							{#if key === 'coverPath'}
								{@const url =
									choices.coverPath === 'source'
										? ((source as AdminAlbumResponse | null)?.coverUrl ?? null)
										: ((target as AdminAlbumResponse).coverUrl ?? null)}
								{#if url}
									<img src={url} alt="" class="h-10 w-10 rounded border object-cover" />
								{:else}
									<span class="font-mono">—</span>
								{/if}
							{:else}
								<span class="text-right font-mono">
									{key === 'artistId'
										? choices.artistId === 'target'
											? artistLabel('target')
											: choices.artistId === 'source'
												? artistLabel('source')
												: customs.artistId || '(none)'
										: resolved(key) || '—'}
								</span>
							{/if}
						</div>
					{/each}
				</div>
			</div>
		{/if}

		<Dialog.Footer>
			{#if step === 'resolve'}
				<Button variant="ghost" onclick={() => (step = 'pick')}>← Back</Button>
			{/if}
			<Button variant="ghost" onclick={() => (open = false)}>Cancel</Button>
			{#if step === 'resolve'}
				<Button onclick={submit} disabled={!source || saving}>
					{saving ? 'Merging…' : 'Merge'}
				</Button>
			{/if}
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
