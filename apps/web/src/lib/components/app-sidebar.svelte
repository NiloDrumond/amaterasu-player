<script lang="ts">
	import { goto } from '$app/navigation';
	import type { CurrentUserResponse } from '$lib/bindings/response/auth/current-user-response';
	import type { RecentPlaylistResponse } from '$lib/bindings/response/playlist/recent-playlist-response';
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Kbd from '$lib/components/ui/kbd/index';
	import { Separator } from '$lib/components/ui/separator';
	import { resetMode, setMode } from 'mode-watcher';
	import type { ComponentProps } from 'svelte';
	import { toast } from 'svelte-sonner';
	import { signOut } from '$lib/services/auth-service';
	import { scanLibrary } from '$lib/services/admin-service';
	import { Icons } from './ui/icons';

	let {
		user,
		recentPlaylists = [],
		ref = $bindable(null),
		...restProps
	}: ComponentProps<typeof Sidebar.Root> & {
		user: CurrentUserResponse;
		recentPlaylists?: RecentPlaylistResponse[];
	} = $props();

	const isAdmin = $derived(user.role === 'admin');

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
			toast.info('Library scan already in progress', {
				description: 'Wait for the current scan to finish before starting another.',
			});
		} else {
			toast.error('Failed to start library scan', {
				description: error,
			});
		}
	}
</script>

<Sidebar.Root {...restProps} bind:ref>
	<Sidebar.Header>
		<Sidebar.MenuButton>
			{#snippet child({ props })}
				<a href="/" {...props}>Amaterasu Player</a>
			{/snippet}
		</Sidebar.MenuButton>
	</Sidebar.Header>
	<Sidebar.Content>
		<Sidebar.Group>
			<Sidebar.Menu>
				<Sidebar.MenuItem>
					<Sidebar.MenuButton>
						{#snippet child({ props })}
							<a href="/search" {...props}
								>Search
								<Kbd.Root>Ctrl + K</Kbd.Root>
							</a>
						{/snippet}
					</Sidebar.MenuButton>
				</Sidebar.MenuItem>
				<Sidebar.MenuItem>
					<Sidebar.MenuButton>
						{#snippet child({ props })}
							<a href="/tracks" {...props}>Tracks</a>
						{/snippet}
					</Sidebar.MenuButton>
				</Sidebar.MenuItem>
				<Sidebar.MenuItem>
					<Sidebar.MenuButton>
						{#snippet child({ props })}
							<a href="/albums" {...props}>Albums</a>
						{/snippet}
					</Sidebar.MenuButton>
				</Sidebar.MenuItem>
				<Sidebar.MenuItem>
					<Sidebar.MenuButton>
						{#snippet child({ props })}
							<a href="/artists" {...props}>Artists</a>
						{/snippet}
					</Sidebar.MenuButton>
				</Sidebar.MenuItem>
				<Sidebar.MenuItem>
					<Sidebar.MenuButton>
						{#snippet child({ props })}
							<a href="/collections" {...props}>Collections</a>
						{/snippet}
					</Sidebar.MenuButton>
				</Sidebar.MenuItem>
				<Sidebar.MenuItem>
					<Sidebar.MenuButton>
						{#snippet child({ props })}
							<a href="/tags" {...props}>Tags</a>
						{/snippet}
					</Sidebar.MenuButton>
				</Sidebar.MenuItem>
			</Sidebar.Menu>
		</Sidebar.Group>
		<Separator class="mx-2 w-auto" />
		<Sidebar.Group>
			<Sidebar.Menu>
				<Sidebar.MenuItem>
					<Sidebar.MenuButton>
						{#snippet child({ props })}
							<a href="/playlists" {...props}>Playlists</a>
						{/snippet}
					</Sidebar.MenuButton>
				</Sidebar.MenuItem>
				{#each recentPlaylists as p (p.id)}
					<Sidebar.MenuItem>
						<Sidebar.MenuButton>
							{#snippet child({ props })}
								<a
									href="/playlists/{p.id}"
									{...props}
									class={(props.class ?? '') + ' pl-6 text-muted-foreground'}
									title={p.name}
								>
									<span class="truncate">{p.name}</span>
								</a>
							{/snippet}
						</Sidebar.MenuButton>
					</Sidebar.MenuItem>
				{/each}
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
								{#if isAdmin}
									<DropdownMenu.Separator />
									<DropdownMenu.Group>
										<DropdownMenu.Label>Admin</DropdownMenu.Label>
										<DropdownMenu.Item onclick={() => window.open('/admin', '_blank', 'noopener')}>
											Admin home <Icons.ExternalLink />
										</DropdownMenu.Item>
										<DropdownMenu.Item onclick={handleScanLibrary}>Scan library</DropdownMenu.Item>
										<DropdownMenu.Sub>
											<DropdownMenu.SubTrigger>Invite users</DropdownMenu.SubTrigger>
											<DropdownMenu.SubContent>
												<DropdownMenu.Item>Email</DropdownMenu.Item>
												<DropdownMenu.Item>Message</DropdownMenu.Item>
												<DropdownMenu.Separator />
												<DropdownMenu.Item>More...</DropdownMenu.Item>
											</DropdownMenu.SubContent>
										</DropdownMenu.Sub>
									</DropdownMenu.Group>
								{/if}
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
