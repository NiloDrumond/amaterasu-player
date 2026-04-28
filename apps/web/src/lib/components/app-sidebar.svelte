<script lang="ts">
	import { goto } from '$app/navigation';
	import type { CurrentUserResponse } from '$lib/bindings/response/auth/current-user-response';
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { resetMode, setMode } from 'mode-watcher';
	import type { ComponentProps } from 'svelte';
	import { toast } from 'svelte-sonner';

	let {
		user,
		ref = $bindable(null),
		...restProps
	}: ComponentProps<typeof Sidebar.Root> & { user: CurrentUserResponse } = $props();

	const isAdmin = $derived(user.role === 'admin');

	async function signOut() {
		await fetch('/api/auth/sign-out', { method: 'POST' });
		goto('/login');
	}

	async function scanLibrary() {
		const res = await fetch('/api/admin/scan-library', { method: 'POST' });
		if (res.ok) {
			toast.success('Library scan started', {
				description: 'Indexing runs in the background.',
			});
		} else if (res.status === 409) {
			toast.info('Library scan already in progress', {
				description: 'Wait for the current scan to finish before starting another.',
			});
		} else {
			toast.error('Failed to start library scan', {
				description: `Server responded with ${res.status}.`,
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
										<DropdownMenu.Item onclick={scanLibrary}>Scan library</DropdownMenu.Item>
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
								<DropdownMenu.Item onclick={signOut}>Sign out</DropdownMenu.Item>
							</DropdownMenu.Content>
						</DropdownMenu.Root>
					{/snippet}
				</Sidebar.MenuButton>
			</Sidebar.MenuItem>
		</Sidebar.Menu>
	</Sidebar.Footer>
	<Sidebar.Rail />
</Sidebar.Root>
