<script lang="ts">
	import { goto } from '$app/navigation';
	import type { CurrentUserResponse } from '$lib/bindings/response/auth/current-user-response';
	import type { ReviewQueueCounts } from '$lib/bindings/response/admin/review-queue-counts';
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { resetMode, setMode } from 'mode-watcher';
	import type { ComponentProps } from 'svelte';
	import { toast } from 'svelte-sonner';
	import { signOut } from '$lib/services/auth-service';
	import { scanLibrary } from '$lib/services/admin-service';
	import { Icons } from '../ui/icons';
	import Badge from '../ui/badge/badge.svelte';

	let {
		user,
		reviewCounts = null,
		ref = $bindable(null),
		...restProps
	}: ComponentProps<typeof Sidebar.Root> & {
		user: CurrentUserResponse;
		reviewCounts?: ReviewQueueCounts | null;
	} = $props();

	let pendingTotal = $derived(
		reviewCounts
			? reviewCounts.pendingAlbums + reviewCounts.pendingTracks + reviewCounts.pendingArtists
			: 0n,
	);

	async function handleSignOut() {
		await signOut(fetch);
		goto('/login');
	}

	async function handleScanLibrary() {
		const { error, status } = await scanLibrary(fetch);
		if (!error) {
			toast.success('Library scan started', {
				description: 'Indexing runs in the background.',
			});
		} else if (status === 409) {
			toast.info('Library scan already in progress');
		} else {
			toast.error('Failed to start library scan', { description: error });
		}
	}
</script>

<Sidebar.Root {...restProps} bind:ref>
	<Sidebar.Header>
		<Sidebar.MenuButton>
			{#snippet child({ props })}
				<a href="/admin" {...props}>Amaterasu Admin</a>
			{/snippet}
		</Sidebar.MenuButton>
	</Sidebar.Header>
	<Sidebar.Content>
		<Sidebar.Group>
			<Sidebar.GroupLabel>Library</Sidebar.GroupLabel>
			<Sidebar.Menu>
				<Sidebar.MenuItem>
					<Sidebar.MenuButton>
						{#snippet child({ props })}
							<a href="/admin/tracks" {...props}>Tracks</a>
						{/snippet}
					</Sidebar.MenuButton>
				</Sidebar.MenuItem>
				<Sidebar.MenuItem>
					<Sidebar.MenuButton>
						{#snippet child({ props })}
							<a href="/admin/albums" {...props}>Albums</a>
						{/snippet}
					</Sidebar.MenuButton>
				</Sidebar.MenuItem>
				<Sidebar.MenuItem>
					<Sidebar.MenuButton>
						{#snippet child({ props })}
							<a href="/admin/artists" {...props}>Artists</a>
						{/snippet}
					</Sidebar.MenuButton>
				</Sidebar.MenuItem>
				<Sidebar.MenuItem>
					<Sidebar.MenuButton>
						{#snippet child({ props })}
							<a href="/admin/tracks/deleted" {...props}>Deleted tracks</a>
						{/snippet}
					</Sidebar.MenuButton>
				</Sidebar.MenuItem>
				<Sidebar.MenuItem>
					<Sidebar.MenuButton>
						{#snippet child({ props })}
							<a href="/admin/users" {...props}>Users</a>
						{/snippet}
					</Sidebar.MenuButton>
				</Sidebar.MenuItem>
				<Sidebar.MenuItem>
					<Sidebar.MenuButton>
						{#snippet child({ props })}
							<a href="/admin/review" {...props}>
								<span>Review</span>
								{#if pendingTotal > 0n}
									<Badge variant="warning">{pendingTotal}</Badge>
								{/if}
							</a>
						{/snippet}
					</Sidebar.MenuButton>
				</Sidebar.MenuItem>
			</Sidebar.Menu>
		</Sidebar.Group>
		<Sidebar.Group>
			<Sidebar.GroupLabel>Player</Sidebar.GroupLabel>
			<Sidebar.Menu>
				<Sidebar.MenuItem>
					<Sidebar.MenuButton>
						{#snippet child({ props })}
							<a href="/" target="_blank" rel="noopener" {...props}
								>Open player <Icons.ExternalLink /></a
							>
						{/snippet}
					</Sidebar.MenuButton>
				</Sidebar.MenuItem>
			</Sidebar.Menu>
		</Sidebar.Group>
	</Sidebar.Content>
	<Sidebar.Footer>
		<Sidebar.Menu>
			<Sidebar.MenuItem>
				<Sidebar.MenuButton size="lg">
					{#snippet child({ props })}
						<DropdownMenu.Root {...props}>
							<DropdownMenu.Trigger>
								{#snippet child({ props })}
									<Sidebar.MenuButton size="lg" {...props}>{user.name}</Sidebar.MenuButton>
								{/snippet}
							</DropdownMenu.Trigger>
							<DropdownMenu.Content class="w-56" align="start">
								<DropdownMenu.Group>
									<DropdownMenu.Sub>
										<DropdownMenu.SubTrigger>Theme</DropdownMenu.SubTrigger>
										<DropdownMenu.SubContent>
											<DropdownMenu.Item onclick={() => setMode('light')}>Light</DropdownMenu.Item>
											<DropdownMenu.Item onclick={() => setMode('dark')}>Dark</DropdownMenu.Item>
											<DropdownMenu.Item onclick={() => resetMode()}>System</DropdownMenu.Item>
										</DropdownMenu.SubContent>
									</DropdownMenu.Sub>
								</DropdownMenu.Group>
								<DropdownMenu.Separator />
								<DropdownMenu.Item onclick={handleScanLibrary}>Scan library</DropdownMenu.Item>
								<DropdownMenu.Separator />
								<DropdownMenu.Item onclick={handleSignOut}>Sign out</DropdownMenu.Item>
							</DropdownMenu.Content>
						</DropdownMenu.Root>
					{/snippet}
				</Sidebar.MenuButton>
			</Sidebar.MenuItem>
		</Sidebar.Menu>
	</Sidebar.Footer>
	<Sidebar.Rail />
</Sidebar.Root>
